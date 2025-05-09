
use tokio::{net::TcpStream, time::Sleep};

use crate::data::{clientbound, serverbound, AsyncReader, AsyncWriter, DataError, Packet, Reader, TextComponentBuilder, Writer};


#[derive(Debug)]
pub enum PacketError {
	WrongPacketID,
	Data(DataError),
	NextStateIncorrect,
	OptionIsNone,
}

impl From<DataError> for PacketError {
	fn from(err: DataError) -> Self {
		PacketError::Data(err)
	}
}

pub struct Connection {
	pub stream: TcpStream,

	pub version: Option<i32>,
	pub host: Option<String>,
	pub port: Option<u16>,

	pub username: Option<String>,
	pub uuid: Option<u128>,

	pub threshold: Option<usize>,
}

impl Connection {
	pub fn new(stream: TcpStream) -> Self {
		Connection {
			stream,
			version: None,
			host: None,
			port: None,
			username: None,
			uuid: None,
			threshold: None,
		}
	}

	pub async fn read_packet(&mut self) -> Result<Packet, PacketError> {
		Ok(self.stream.read_packet(self.threshold).await?)
	}

	pub async fn write_packet(&mut self, packet: Packet) -> Result<(), PacketError> {
		Ok(self.stream.write_packet(packet, self.threshold).await?)
	}
}

pub async fn main(stream: TcpStream) {
	let mut conn = Connection::new(stream);
	let Ok(addr) = conn.stream.peer_addr() else { return; };
	println!("Подключение: {addr}");
	// читаем первый пакет
	match read_first_packet(&mut conn).await {
		Ok(_) => {}, Err(e) => println!("Ошибка во время обработки пакета: {e:?}")
	}
	println!("Отключение: {addr}");
	println!();
}

async fn read_first_packet(conn: &mut Connection) -> Result<(), PacketError> {
	let mut packet = conn.stream.read_packet(None).await?;
	if packet.id() != 0 { return Err(PacketError::WrongPacketID);}
	conn.version = Some(packet.read_varint()?);
	conn.host = Some(packet.read_string()?);
	conn.port = Some(packet.read_short()?);
	let ns = packet.read_varint()?;

	match ns {
		1 => return the_status(conn).await,
		2 => the_login(conn).await?,
		_ => return Err(PacketError::NextStateIncorrect)
	};

	the_configuration(conn).await?;
	Ok(())
}

async fn the_status(conn: &mut Connection) -> Result<(), PacketError> {
	let packet = conn.stream.read_packet(None).await?;
	if packet.id() != 0 { return Err(PacketError::WrongPacketID); }
	let mut p = Packet::empty(clientbound::status::RESPONSE);
	let status = "{
		\"version\": {
			\"name\": \"1.21.5\",
			\"protocol\": 770
		},
		\"players\": {
			\"max\": 0,
			\"online\": 1
		}
	}";
	p.write_string(status)?;
	conn.write_packet(p).await?;

	let mut packet = conn.stream.read_packet(None).await?;
	if packet.id() != 1 { return Err(PacketError::WrongPacketID); }
	let mut p = Packet::empty(clientbound::status::PONG_RESPONSE);
	p.write_long(packet.read_long()?)?;
	conn.write_packet(p).await?;

	Ok(())
}

async fn the_login(conn: &mut Connection) -> Result<(), PacketError> {

	if conn.version != Some(770) {
		let mut packet = Packet::empty(clientbound::login::DISCONNECT);
		let component =
			TextComponentBuilder::new()
			.text("Версия игры отличается от 1.21.5")
			.color("red")
			.build();
		packet.write_string(&component.as_json()?)?;
		return conn.write_packet(packet).await;
	}

	let mut packet = conn.read_packet().await?;
	if packet.id() != serverbound::login::START { return Err(PacketError::WrongPacketID); }
	conn.username = Some(packet.read_string()?);
	conn.uuid = Some(packet.read_uuid()?);

	// println!("Адрес клиента: {}", stream.peer_addr().unwrap());
	// println!("Адрес сервера: {}:{}", data.1, data.2);
	// println!("Username: {username}\nUUID: {:X}", uuid);

	let threshold = 512usize;
	let mut packet = Packet::empty(clientbound::login::SET_COMPRESSION);
	packet.write_varint(threshold as i32)?;
	conn.write_packet(packet).await?;
	conn.threshold = Some(threshold);

	let mut packet = Packet::empty(clientbound::login::SUCCESS);
	packet.write_uuid(conn.uuid.ok_or(PacketError::OptionIsNone)?)?;
	packet.write_string(conn.username.clone().ok_or(PacketError::OptionIsNone)?.as_ref())?;
	packet.write_varint(0)?;
	conn.write_packet(packet).await?;

	let packet = conn.read_packet().await?;
	if packet.id() != serverbound::login::ACKNOWLEDGED { return Err(PacketError::WrongPacketID); }

	Ok(())
}

async fn the_configuration(conn: &mut Connection) -> Result<(), PacketError> {
	let packet = Packet::empty(clientbound::configuration::FINISH);
	conn.write_packet(packet).await?;
	loop {

	}
}