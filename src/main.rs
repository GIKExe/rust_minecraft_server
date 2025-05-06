use std::{env::args, path::PathBuf, sync::Arc};

use log::{debug, error, info};
use rust_mc_proto::Packet;
use rust_mc_serv::{
	ServerError,
	config::Config,
	context::ServerContext,
	data::text_component::TextComponent,
	event::{Listener, PacketHandler},
	player::context::ClientContext,
	protocol::ConnectionState,
	start_server,
};

struct ExampleListener;

impl Listener for ExampleListener {
	fn on_status(
		&self,
		client: Arc<ClientContext>,
		response: &mut String,
	) -> Result<(), ServerError> {
		*response = format!(
			"{{
				\"version\": {{
					\"name\": \"idk\",
					\"protocol\": {}
				}},
				\"players\": {{
					\"max\": 100,
					\"online\": 42,
					\"sample\": [
						{{
							\"name\": \"Жопа\",
							\"id\": \"00000000-0000-0000-0000-000000000000\"
						}}
					]
				}},
				\"description\": {},
				\"favicon\": \"data:image/png;base64,<data>\",
				\"enforcesSecureChat\": false
			}}",
			client.handshake().unwrap().protocol_version,
			TextComponent::builder()
				.text("Hello World! ")
				.extra(vec![
					TextComponent::builder()
						.text("Protocol: ")
						.color("gold")
						.extra(vec![
							TextComponent::builder()
								.text(&client.handshake().unwrap().protocol_version.to_string())
								.underlined(true)
								.build()
						])
						.build(),
					TextComponent::builder()
						.text("\nServer Addr: ")
						.color("green")
						.extra(vec![
							TextComponent::builder()
								.text(&format!(
									"{}:{}",
									client.handshake().unwrap().server_address,
									client.handshake().unwrap().server_port
								))
								.underlined(true)
								.build()
						])
						.build()
				])
				.build()
				.as_json()?
		);

		Ok(())
	}
}

struct ExamplePacketHandler;

impl PacketHandler for ExamplePacketHandler {
	fn on_incoming_packet(
		&self,
		client: Arc<ClientContext>,
		packet: &mut Packet,
		_: &mut bool,
		state: ConnectionState,
	) -> Result<(), ServerError> {
		debug!(
			"{} -> S\t| 0x{:02x}\t| {:?}\t| {} bytes",
			client.addr.clone(),
			packet.id(),
			state,
			packet.len()
		);

		Ok(())
	}

	fn on_outcoming_packet(
		&self,
		client: Arc<ClientContext>,
		packet: &mut Packet,
		_: &mut bool,
		state: ConnectionState,
	) -> Result<(), ServerError> {
		debug!(
			"{} <- S\t| 0x{:02x}\t| {:?}\t| {} bytes",
			client.addr.clone(),
			packet.id(),
			state,
			packet.len()
		);

		Ok(())
	}
}

fn main() {
	// Инициализируем логи
	// Чтобы читать debug-логи, юзаем `RUST_LOG=debug cargo run`
	colog::init();

	// Получение аргументов
	let exec = args().next().expect("Неизвестная система");
	let args = args().skip(1).collect::<Vec<String>>();

	if args.len() > 1 {
		info!("Использование: {exec} [путь до файла конфигурации]");
		return;
	}

	// Берем путь из аргумента либо по дефолту берем "./server.toml"
	let config_path = PathBuf::from(args.get(0).unwrap_or(&"server.toml".to_string()));

	// Чтение конфига, если ошибка - выводим
	let config = match Config::load_from_file(config_path) {
		Some(config) => config,
		None => {
			error!("Ошибка чтения конфигурации");
			return;
		}
	};

	// Делаем немутабельную потокобезопасную ссылку на конфиг
	// Впринципе можно и просто клонировать сам конфиг в каждый сука поток ебать того рот ебать блять
	// но мы этого делать не будем чтобы не было мемори лик лишнего
	let config = Arc::new(config);

	// Создаем контекст сервера
	// Передается во все подключения
	let mut server = ServerContext::new(config);

	server.add_listener(Box::new(ExampleListener)); // Добавляем пример листенера
	server.add_packet_handler(Box::new(ExamplePacketHandler)); // Добавляем пример пакет хандлера

	// Бетонируем сервер контекст от изменений
	let server = Arc::new(server);

	// Запускаем сервер из специально отведенной под это дело функцией
	start_server(server);
}
