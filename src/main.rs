
use std::{io::Cursor, net::Shutdown};

use data::{DataError, DataReader};
use inet::Server;
use tokio::{io::AsyncWriteExt, net::TcpStream};

mod data;
mod inet;

#[tokio::main]
async fn main() {
	let Ok(server) = Server::new("127.0.0.1:25565").await else {
		println!("Не удалось забиндить сервер");
		return;
	};

	loop {
		let stream = server.accept().await;
		tokio::spawn(test(stream));
	}
}

async fn test(mut stream: TcpStream) {
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

async fn read_first_packet(stream: &mut TcpStream) -> Result<(), DataError> {
	let size = stream.read_varint().await?;
	let mut buf = Cursor::new(stream.read_bytes(size as usize).await?);
	let id = buf.read_varint().await?;
	let version = buf.read_varint().await?;
	let host = buf.read_string().await?;
	let port = buf.read_short().await?;
	let ns = buf.read_varint().await?;
	println!("Айди пакета: {id}");
	println!("Версия протокола: {version}");
	println!("Адрес сервера: {host}:{port}");
	println!("Следующее состояние: {ns}");
	Ok(())
}
