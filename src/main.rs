

use std::fs;

use data::{Buffer, NBT_Reader, Reader};
use inet::Server;

mod data;
mod inet;
mod cycle;

#[tokio::main]
async fn main() {
	// let Ok(server) = Server::new("127.0.0.1:25565").await else {
	// 	println!("Не удалось забиндить сервер");
	// 	return;
	// };

	// loop {
	// 	let stream = server.accept().await;
	// 	tokio::spawn(cycle::main(stream));
	// }

	let mut buf = Buffer::new(fs::read("servers.dat").unwrap());

	// let mut buf = Buffer::new(vec![
	// 	0x0a,
	// 	// 0x00, 0x0b,
	// 	// 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
	// 	0x08,
	// 	0x00, 0x04,
	// 	0x6e, 0x61, 0x6d, 0x65,
	// 	0x00, 0x05,
	// 	0x42, 0x61, 0x6e, 0x61, 0x6e,
	// 	0x00
	// ]);

	let nbt = buf.read_nbt(false).unwrap();
	println!("{nbt:?}");
}

