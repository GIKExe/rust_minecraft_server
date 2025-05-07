use std::{io::Cursor, sync::Arc, time::Duration};

use data::{DataError, DataReader};
use inet::Server;
use tokio::{io::AsyncReadExt, net::TcpStream, time::sleep};



mod inet;
mod data;



#[tokio::main]
async fn main() {
	let Ok(server) = Server::new("127.0.0.1:25565").await else {
		println!("Не удалось забиндить сервер"); return;
	};

	loop {
		let stream = server.accept().await;
		tokio::spawn(test(stream));
	}
}

async fn test(stream: TcpStream) {
	let Ok(addr) = stream.peer_addr() else {return;};
	println!("Подключение: {addr}");
	// читаем первый пакет
	let size = stream.read_varint();
	println!("Отключение: {addr}");
}