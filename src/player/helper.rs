use std::{
	io::Read,
	sync::Arc,
	time::{Duration, SystemTime},
};

use rust_mc_proto::{DataReader, DataWriter, Packet};

use crate::{
	ServerError,
	data::{ReadWriteNBT, text_component::TextComponent},
	protocol::{
		id::{clientbound, serverbound},
		*,
	},
};

use super::context::ClientContext;

// Помощник в работе с протоколом
// Может быть использован где угодно, но сделан именно для листенеров и пакет хандлеров
// Через него удобно делать всякую одинаковую херь
// Возможно надо было бы сделать прям обязательный какойто структ через который только можно было отправлять пакеты ...
// ... но мне лень
// Пусть юзают подключение и отправляют пакеты через него если хотят
// Почему бы и нет если да
pub struct ProtocolHelper {
	client: Arc<ClientContext>,
	state: ConnectionState,
}

impl ProtocolHelper {
	pub fn new(client: Arc<ClientContext>) -> Self {
		Self {
			state: client.state(),
			client,
		}
	}

	pub fn reset_chat(&self) -> Result<(), ServerError> {
		match self.state {
			ConnectionState::Configuration => {
				self
					.client
					.write_packet(&Packet::empty(clientbound::configuration::RESET_CHAT))?;
				Ok(())
			}
			_ => Err(ServerError::UnexpectedState),
		}
	}

	pub fn store_cookie(&self, id: &str, data: &[u8]) -> Result<(), ServerError> {
		self.client.write_packet(&Packet::build(
			match self.state {
				ConnectionState::Configuration => clientbound::configuration::STORE_COOKIE,
				ConnectionState::Play => clientbound::play::STORE_COOKIE,
				_ => return Err(ServerError::UnexpectedState),
			},
			|p| {
				p.write_string(id)?;
				p.write_bytes(data)
			},
		)?)?;
		Ok(())
	}

	/// Leave from Configuration to Play state
	pub fn leave_configuration(&self) -> Result<(), ServerError> {
		match self.state {
			ConnectionState::Configuration => {
				self
					.client
					.write_packet(&Packet::empty(clientbound::configuration::FINISH))?;
				self
					.client
					.read_packet(&[serverbound::configuration::ACKNOWLEDGE_FINISH])?;
				self.client.set_state(ConnectionState::Play)?;
				Ok(())
			}
			_ => Err(ServerError::UnexpectedState),
		}
	}

	/// Enter to Configuration from Play state
	pub fn enter_configuration(&self) -> Result<(), ServerError> {
		match self.state {
			ConnectionState::Play => {
				self
					.client
					.write_packet(&Packet::empty(clientbound::play::START_CONFIGURATION))?;
				self
					.client
					.read_packet(&[serverbound::play::ACKNOWLEDGE_CONFIGURATION])?;
				self.client.set_state(ConnectionState::Configuration)?;
				Ok(())
			}
			_ => Err(ServerError::UnexpectedState),
		}
	}

	/// Enter to Configuration from Play state
	pub fn ping(&self) -> Result<Duration, ServerError> {
		match self.state {
			ConnectionState::Play => {
				let time = SystemTime::now();
				self
					.client
					.write_packet(&Packet::empty(clientbound::play::PING))?;
				self.client.read_packet(&[serverbound::play::PONG])?;
				Ok(SystemTime::now().duration_since(time).unwrap())
			}
			ConnectionState::Configuration => {
				let time = SystemTime::now();
				self
					.client
					.write_packet(&Packet::empty(clientbound::configuration::PING))?;
				self
					.client
					.read_packet(&[serverbound::configuration::PONG])?;
				Ok(SystemTime::now().duration_since(time).unwrap())
			}
			_ => Err(ServerError::UnexpectedState),
		}
	}

	pub fn disconnect(&self, reason: TextComponent) -> Result<(), ServerError> {
		let packet = match self.state {
			ConnectionState::Login => {
				let text = reason.as_json()?;
				Packet::build(0x00, |p| p.write_string(&text))?
			}
			ConnectionState::Configuration => {
				let mut packet = Packet::empty(0x02);
				packet.write_nbt(&reason)?;
				packet
			}
			ConnectionState::Play => {
				let mut packet = Packet::empty(0x1C);
				packet.write_nbt(&reason)?;
				packet
			}
			_ => {
				self.client.close();
				return Ok(());
			}
		};
		self.client.write_packet(&packet)?;
		Ok(())
	}

	/// Returns cookie content
	pub fn request_cookie(&self, id: &str) -> Result<Option<Vec<u8>>, ServerError> {
		match self.state {
			ConnectionState::Configuration => {
				let mut packet = Packet::empty(clientbound::configuration::COOKIE_REQUEST);
				packet.write_string(id)?;
				self.client.write_packet(&packet)?;

				let mut packet = self
					.client
					.read_packet(&[serverbound::configuration::COOKIE_RESPONSE])?;
				packet.read_string()?;
				let data = if packet.read_boolean()? {
					let n = packet.read_usize_varint()?;
					Some(packet.read_bytes(n)?)
				} else {
					None
				};

				Ok(data)
			}
			ConnectionState::Play => {
				let mut packet = Packet::empty(clientbound::play::COOKIE_REQUEST);
				packet.write_string(id)?;
				self.client.write_packet(&packet)?;

				let mut packet = self
					.client
					.read_packet(&[serverbound::play::COOKIE_RESPONSE])?;
				packet.read_string()?;
				let data = if packet.read_boolean()? {
					let n = packet.read_usize_varint()?;
					Some(packet.read_bytes(n)?)
				} else {
					None
				};

				Ok(data)
			}
			_ => Err(ServerError::UnexpectedState),
		}
	}

	/// Returns login plugin response - (message_id, payload)
	pub fn send_login_plugin_request(
		&self,
		id: i32,
		channel: &str,
		data: &[u8],
	) -> Result<(i32, Option<Vec<u8>>), ServerError> {
		match self.state {
			ConnectionState::Login => {
				let mut packet = Packet::empty(clientbound::login::PLUGIN_REQUEST);
				packet.write_varint(id)?;
				packet.write_string(channel)?;
				packet.write_bytes(data)?;
				self.client.write_packet(&packet)?;

				let mut packet = self
					.client
					.read_packet(&[serverbound::login::PLUGIN_RESPONSE])?;
				let identifier = packet.read_varint()?;
				let data = if packet.read_boolean()? {
					let mut data = Vec::new();
					packet.get_mut().read_to_end(&mut data).unwrap();
					Some(data)
				} else {
					None
				};

				Ok((identifier, data))
			}
			_ => Err(ServerError::UnexpectedState),
		}
	}

	pub fn send_plugin_message(&self, channel: &str, data: &[u8]) -> Result<(), ServerError> {
		let mut packet = match self.state {
			ConnectionState::Configuration => Packet::empty(clientbound::configuration::PLUGIN_MESSAGE),
			ConnectionState::Play => Packet::empty(clientbound::play::PLUGIN_MESSAGE),
			_ => return Err(ServerError::UnexpectedState),
		};
		packet.write_string(channel)?;
		packet.write_bytes(data)?;
		self.client.write_packet(&packet)?;
		Ok(())
	}
}
