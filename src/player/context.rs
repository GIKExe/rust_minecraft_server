use std::{
	collections::VecDeque,
	hash::Hash,
	net::{SocketAddr, TcpStream},
	sync::{
		Arc, Mutex, RwLock,
		atomic::{AtomicBool, Ordering},
	},
	thread,
	time::Duration,
};

use rust_mc_proto::{MinecraftConnection, Packet};
use uuid::Uuid;

use super::helper::ProtocolHelper;
use crate::{ServerError, context::ServerContext, protocol::ConnectionState};

// Клиент контекст
// Должен быть обернут в Arc для передачи между потоками
pub struct ClientContext {
	pub server: Arc<ServerContext>,
	pub addr: SocketAddr,
	conn: RwLock<MinecraftConnection<TcpStream>>,
	handshake: RwLock<Option<Handshake>>,
	client_info: RwLock<Option<ClientInfo>>,
	player_info: RwLock<Option<PlayerInfo>>,
	state: RwLock<ConnectionState>,
	packet_buffer: Mutex<VecDeque<Packet>>,
	read_loop: AtomicBool,
	is_alive: AtomicBool,
	position: RwLock<(f64, f64, f64)>,
	velocity: RwLock<(f64, f64, f64)>,
	rotation: RwLock<(f32, f32)>,
}

// Реализуем сравнение через адрес
// IPv4 не должен обманывать, иначе у нас случится коллапс
impl PartialEq for ClientContext {
	fn eq(&self, other: &Self) -> bool {
		self.addr == other.addr
	}
}

impl Hash for ClientContext {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.addr.hash(state);
	}
}

impl Eq for ClientContext {}

impl ClientContext {
	pub fn new(server: Arc<ServerContext>, conn: MinecraftConnection<TcpStream>) -> ClientContext {
		ClientContext {
			server,
			addr: conn.get_ref().peer_addr().unwrap(),
			conn: RwLock::new(conn),
			handshake: RwLock::new(None),
			client_info: RwLock::new(None),
			player_info: RwLock::new(None),
			state: RwLock::new(ConnectionState::Handshake),
			packet_buffer: Mutex::new(VecDeque::new()),
			read_loop: AtomicBool::new(false),
			is_alive: AtomicBool::new(true),
			position: RwLock::new((0.0, 0.0, 0.0)),
			velocity: RwLock::new((0.0, 0.0, 0.0)),
			rotation: RwLock::new((0.0, 0.0)),
		}
	}

	pub fn set_handshake(self: &Arc<Self>, handshake: Handshake) {
		*self.handshake.write().unwrap() = Some(handshake);
	}

	pub fn set_client_info(self: &Arc<Self>, client_info: ClientInfo) {
		*self.client_info.write().unwrap() = Some(client_info);
	}

	pub fn set_player_info(self: &Arc<Self>, player_info: PlayerInfo) {
		*self.player_info.write().unwrap() = Some(player_info);
	}

	pub fn set_state(self: &Arc<Self>, state: ConnectionState) -> Result<(), ServerError> {
		*self.state.write().unwrap() = state.clone();

		for handler in self
			.server
			.packet_handlers(|o| o.on_state_priority())
			.iter()
		{
			handler.on_state(self.clone(), state.clone())?;
		}

		Ok(())
	}

	pub fn handshake(self: &Arc<Self>) -> Option<Handshake> {
		self.handshake.read().unwrap().clone()
	}

	pub fn client_info(self: &Arc<Self>) -> Option<ClientInfo> {
		self.client_info.read().unwrap().clone()
	}

	pub fn player_info(self: &Arc<Self>) -> Option<PlayerInfo> {
		self.player_info.read().unwrap().clone()
	}

	pub fn state(self: &Arc<Self>) -> ConnectionState {
		self.state.read().unwrap().clone()
	}

	pub fn set_position(self: &Arc<Self>, position: (f64, f64, f64)) {
		*self.position.write().unwrap() = position;
	}

	pub fn set_velocity(self: &Arc<Self>, velocity: (f64, f64, f64)) {
		*self.velocity.write().unwrap() = velocity;
	}

	pub fn set_rotation(self: &Arc<Self>, rotation: (f32, f32)) {
		*self.rotation.write().unwrap() = rotation;
	}

	pub fn position(self: &Arc<Self>) -> (f64, f64, f64) {
		self.position.read().unwrap().clone()
	}

	pub fn velocity(self: &Arc<Self>) -> (f64, f64, f64) {
		self.velocity.read().unwrap().clone()
	}

	pub fn rotation(self: &Arc<Self>) -> (f32, f32) {
		self.rotation.read().unwrap().clone()
	}

	pub fn write_packet(self: &Arc<Self>, packet: &Packet) -> Result<(), ServerError> {
		let state = self.state();
		let mut packet = packet.clone();
		let mut cancelled = false;
		for handler in self
			.server
			.packet_handlers(|o| o.on_outcoming_packet_priority())
			.iter()
		{
			handler.on_outcoming_packet(self.clone(), &mut packet, &mut cancelled, state.clone())?;
			packet.get_mut().set_position(0);
		}
		if !cancelled {
			match self.conn.write().unwrap().write_packet(&packet) {
				Ok(_) => {}
				Err(e) => {
					self.is_alive.store(false, Ordering::SeqCst);
					return Err(e.into());
				}
			};
		}
		Ok(())
	}

	pub fn run_read_loop(self: &Arc<Self>) -> Result<(), ServerError> {
		self.read_loop.store(true, Ordering::SeqCst);

		let mut conn = self.conn.read().unwrap().try_clone()?; // так можно делать т.к сокет это просто поинтер

		while self.is_alive() {
			let mut packet = match conn.read_packet() {
				Ok(v) => v,
				Err(e) => {
					self.is_alive.store(false, Ordering::SeqCst);
					return Err(e.into());
				}
			};
			let mut cancelled = false;
			let state = self.state();
			for handler in self
				.server
				.packet_handlers(|o| o.on_incoming_packet_priority())
				.iter()
			{
				handler.on_incoming_packet(self.clone(), &mut packet, &mut cancelled, state.clone())?;
				packet.get_mut().set_position(0);
			}
			if !cancelled {
				self.packet_buffer.lock().unwrap().push_back(packet);
			}
		}

		Ok(())
	}

	/// Please avoid using of this bullshit
	pub fn read_any_packet(self: &Arc<Self>) -> Result<Packet, ServerError> {
		if self.read_loop.load(Ordering::SeqCst) {
			loop {
				if let Some(packet) = self.packet_buffer.lock().unwrap().pop_front() {
					return Ok(packet);
				}
				thread::sleep(Duration::from_millis(4));
			}
		} else {
			let state = self.state();

			loop {
				let mut packet = match self.conn.write().unwrap().read_packet() {
					Ok(v) => v,
					Err(e) => {
						self.is_alive.store(false, Ordering::SeqCst);
						return Err(e.into());
					}
				};
				let mut cancelled = false;
				for handler in self
					.server
					.packet_handlers(|o| o.on_incoming_packet_priority())
					.iter()
				{
					handler.on_incoming_packet(self.clone(), &mut packet, &mut cancelled, state.clone())?;
					packet.get_mut().set_position(0);
				}
				if !cancelled {
					break Ok(packet);
				}
			}
		}
	}

	pub fn read_packet(self: &Arc<Self>, ids: &[u8]) -> Result<Packet, ServerError> {
		if self.read_loop.load(Ordering::SeqCst) {
			loop {
				{
					let mut locked = self.packet_buffer.lock().unwrap();
					for (i, packet) in locked.clone().iter().enumerate() {
						if ids.contains(&packet.id()) {
							locked.remove(i);
							return Ok(packet.clone());
						}
					}
				}
				thread::sleep(Duration::from_millis(4));
			}
		} else {
			let packet = match self.read_any_packet() {
				Ok(v) => v,
				Err(e) => {
					self.is_alive.store(false, Ordering::SeqCst);
					return Err(e);
				}
			};

			if ids.contains(&packet.id()) {
				Ok(packet)
			} else {
				Err(ServerError::UnexpectedPacket(packet.id()))
			}
		}
	}

	pub fn push_packet_back(self: &Arc<Self>, packet: Packet) {
		self.packet_buffer.lock().unwrap().push_back(packet)
	}

	pub fn close(self: &Arc<Self>) {
		self.conn.write().unwrap().close();
	}

	pub fn set_compression(self: &Arc<Self>, threshold: Option<usize>) {
		self.conn.write().unwrap().set_compression(threshold);
	}

	pub fn is_alive(self: &Arc<Self>) -> bool {
		self.is_alive.load(Ordering::SeqCst)
	}

	pub fn protocol_helper(self: &Arc<Self>) -> ProtocolHelper {
		ProtocolHelper::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Handshake {
	pub protocol_version: i32,
	pub server_address: String,
	pub server_port: u16,
}

#[derive(Clone)]
pub struct ClientInfo {
	pub brand: String,
	pub locale: String,
	pub view_distance: i8,
	pub chat_mode: i32,
	pub chat_colors: bool,
	pub displayed_skin_parts: u8,
	pub main_hand: i32,
	pub enable_text_filtering: bool,
	pub allow_server_listings: bool,
	pub particle_status: i32,
}

#[derive(Clone)]
pub struct PlayerInfo {
	pub name: String,
	pub uuid: Uuid,
}
