
use tokio::net::TcpStream;

use crate::data::{clientbound, serverbound, AsyncReader, AsyncWriter, DataError, Packet, Reader, TextComponentBuilder, Writer};


#[derive(Debug)]
pub enum PacketError {
	WrongPacketID,
	Data(DataError),
	NextStateIncorrect,
}

impl From<DataError> for PacketError {
	fn from(err: DataError) -> Self {
		PacketError::Data(err)
	}
}

pub async fn main(mut stream: TcpStream) {
	let Ok(addr) = stream.peer_addr() else {
		return;
	};
	println!("Подключение: {addr}");
	// читаем первый пакет
	match read_first_packet(&mut stream).await {
		Ok(_) => {}, Err(e) => println!("Ошибка во время обработки пакета: {e:?}")
	}
	println!("Отключение: {addr}");
	println!();
}

async fn read_first_packet(stream: &mut TcpStream) -> Result<(), PacketError> {
	let mut packet = stream.read_packet(None).await?;
	if packet.id() != 0 { return Err(PacketError::WrongPacketID);}
	let version = packet.read_varint()?;
	let host = packet.read_string()?;
	let port = packet.read_short()?;
	let ns = packet.read_varint()?;

	if version != 770 {
		let mut packet = Packet::empty(0x00);
		let component =
			TextComponentBuilder::new()
			.text("Версия игры отличается от 1.21.5")
			.color("red")
			.build();
		packet.write_string(&component.as_json()?)?;
		return Ok(stream.write_packet(packet, None).await?);
	}

	match ns {
		1 => the_status(stream).await,
		2 => the_login(stream, (version, host, port)).await,
		_ => Err(PacketError::NextStateIncorrect)
	}
}

async fn the_status(stream: &mut TcpStream) -> Result<(), PacketError> {
	let packet = stream.read_packet(None).await?;
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
	stream.write_packet(p, None).await?;

	let mut packet = stream.read_packet(None).await?;
	if packet.id() != 1 { return Err(PacketError::WrongPacketID); }
	let mut p = Packet::empty(clientbound::status::PONG_RESPONSE);
	p.write_long(packet.read_long()?)?;
	stream.write_packet(p, None).await?;

	Ok(())
}

async fn the_login(stream: &mut TcpStream, data: (i32, String, u16)) -> Result<(), PacketError> {

	if data.0 != 770 {
		let mut packet = Packet::empty(clientbound::login::DISCONNECT);
		let component =
			TextComponentBuilder::new()
			.text("Версия игры отличается от 1.21.5")
			.color("red")
			.build();
		packet.write_string(&component.as_json()?)?;
		return Ok(stream.write_packet(packet, None).await?);
	}

	// println!("Версия протокола: {}", data.0);
	// println!("Адрес сервера: {}:{}", data.1, data.2);

	// let mut packet = Packet::empty(clientbound::login::DISCONNECT);
	// let component =
	// 	TextComponentBuilder::new()
	// 	.text("Вы кто такие? Я вас не звал. Идите нахуй.")
	// 	.color("red")
	// 	.build();
	// packet.write_string(&component.as_json()?).await?;
	// return Ok(stream.write_packet(packet, None).await?);

	let mut packet = stream.read_packet(None).await?;
	if packet.id() != serverbound::login::START { return Err(PacketError::WrongPacketID); }
	let username = packet.read_string()?;
	let uuid = packet.read_uuid()?;

	println!("Адрес клиента: {:?}", stream.peer_addr());
	println!("Адрес сервера: {}:{}", data.1, data.2);
	println!("Username: {username}\n UUID: {:X}", uuid);

	let mut packet = Packet::empty(clientbound::login::SET_COMPRESSION);
	packet.write_varint(512)?;

	Ok(())
}