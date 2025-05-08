

use inet::Server;


mod data;
mod inet;
mod cycle;

#[tokio::main]
async fn main() {
	let Ok(server) = Server::new("127.0.0.1:25565").await else {
		println!("Не удалось забиндить сервер");
		return;
	};

	loop {
		let stream = server.accept().await;
		tokio::spawn(cycle::main(stream));
	}
}

