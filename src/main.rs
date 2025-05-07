use std::{sync::Arc, time::Duration};

use data::{DataError, DataReader};
use inet::Server;
use tokio::{net::TcpStream, time::sleep};



mod inet;
mod data;



#[tokio::main]
async fn main() {
	let Ok(server) = Server::new("127.0.0.1:25565").await else {
		println!("Не удалось забиндить сервер"); return;
	};

	loop {
		let stream = server.accept().await;
		tokio::spawn(test(Arc::new(stream)));
	}
}

async fn test(stream: Arc<TcpStream>) {
	let Ok(addr) = stream.peer_addr() else {return;};
	println!("Подключение: {addr}");
	read_first_packet(stream.clone());
	println!("Отключение: {addr}");
}

fn read_first_packet(stream: Arc<TcpStream>) -> Result<(), DataError>{
	let size = stream.read_varint()?;
}