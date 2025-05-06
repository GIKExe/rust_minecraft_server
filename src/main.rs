
use data::DataReader;
use inet::Server;

mod data;
mod inet;

pub fn main() {
	let Ok(server) = Server::new("127.0.0.1", 25565) else {
		println!("Не удалось забиндить сервер"); return;
	};

	loop {
		let mut stream = server.accept();
		let addr = stream.peer_addr().unwrap();
		println!("Подключение: {addr}");
		let data = match stream.read_bytes(4) {
			Ok(data) => data,
			Err(e) => {
				println!("Ошибка чтения: {e:?}"); continue;
			}
		};
		println!("Данные получены: {}", String::from_utf8_lossy(&data));
		println!("Отключение: {addr}");
	}
}



