use std::{io::Read, sync::Arc};

use crate::{
	ServerError,
	player::context::{ClientContext, ClientInfo, Handshake, PlayerInfo},
};
use rust_mc_proto::{DataReader, DataWriter, Packet};

use crate::trigger_event;

use super::{
	ConnectionState,
	id::*,
	play::{handle_configuration_state, handle_play_state},
};

pub const BRAND: &str = "rust_mc_serv";

pub fn handle_connection(
	client: Arc<ClientContext>, // Контекст клиента
) -> Result<(), ServerError> {
	// Чтение рукопожатия
	// Получение пакетов производится через client.conn(),
	// ВАЖНО: не помещать сам client.conn() в переменные,
	// он должен сразу убиваться иначе соединение гдето задедлочится
	let mut packet = client.read_packet(&[serverbound::handshake::HANDSHAKE])?;

	let protocol_version = packet.read_varint()?; // Получаем версия протокола, может быть отрицательным если наш клиент дэбил
	let server_address = packet.read_string()?; // Получаем домен/адрес сервера к которому пытается подключиться клиент, например "play.example.com", а не айпи
	let server_port = packet.read_unsigned_short()?; // Все тоже самое что и с адресом сервера и все потому же и за тем же
	let next_state = packet.read_varint()?; // Тип подключения: 1 для получения статуса и пинга, 2 и 3 для обычного подключения

	client.set_handshake(Handshake {
		protocol_version,
		server_address,
		server_port,
	});

	match next_state {
		1 => {
			// Тип подключения - статус
			client.set_state(ConnectionState::Status)?; // Мы находимся в режиме Status

			loop {
				// Чтение запроса
				let mut packet = client.read_any_packet()?;

				match packet.id() {
					serverbound::status::REQUEST => {
						// Запрос статуса
						let mut packet = Packet::empty(clientbound::status::RESPONSE);

						// Дефолтный статус
						let mut status = "{
							\"version\": {
								\"name\": \"Error\",
								\"protocol\": 0
							},
							\"description\": {\"text\": \"Internal server error\"}
						}"
						.to_string();

						// Опрос всех листенеров
						trigger_event!(client, status, &mut status);

						// Отправка статуса
						packet.write_string(&status)?;

						client.write_packet(&packet)?;
					}
					serverbound::status::PING_REQUEST => {
						// Пинг
						// Раньше мы просто отправляли ему его-же пакет, но сейчас,
						// С приходом к власти констант айди-пакетов, нам приходится делать такое непотребство
						let timestamp = packet.read_long()?;
						let mut packet = Packet::empty(clientbound::status::PONG_RESPONSE);
						packet.write_long(timestamp)?;
						client.write_packet(&packet)?;
					}
					id => {
						return Err(ServerError::UnexpectedPacket(id));
					}
				}
			}
		}
		2 => {
			// Тип подключения - игра
			client.set_state(ConnectionState::Login)?; // Мы находимся в режиме Login

			// Читаем пакет Login Start
			let mut packet = client.read_packet(&[serverbound::login::START])?;

			let name = packet.read_string()?;
			let uuid = packet.read_uuid()?;

			client.set_player_info(PlayerInfo {
				name: name.clone(),
				uuid: uuid.clone(),
			});

			if client.server.config.server.online_mode {
				// TODO: encryption packets
			}

			// Отправляем пакет Set Compression если сжатие указано
			if let Some(threshold) = client.server.config.server.compression_threshold {
				client.write_packet(&Packet::build(clientbound::login::SET_COMPRESSION, |p| {
					p.write_usize_varint(threshold)
				})?)?;
				client.set_compression(Some(threshold)); // Устанавливаем сжатие на соединении
			}

			// Отправка пакета Login Success
			client.write_packet(&Packet::build(clientbound::login::SUCCESS, |p| {
				p.write_uuid(&uuid)?;
				p.write_string(&name)?;
				p.write_varint(0)
			})?)?;

			client.read_packet(&[serverbound::login::ACKNOWLEDGED])?; // Пакет Login Acknowledged

			client.set_state(ConnectionState::Configuration)?; // Мы перешли в режим Configuration

			// Получение бренда клиента из Serverbound Plugin Message
			// Identifier канала откуда берется бренд: minecraft:brand
			let brand = loop {
				let mut packet = client.read_packet(&[serverbound::configuration::PLUGIN_MESSAGE])?; // Пакет Serverbound Plugin Message

				let identifier = packet.read_string()?;

				let mut data = Vec::new();
				packet.get_mut().read_to_end(&mut data).unwrap();

				if identifier == "minecraft:brand" {
					break String::from_utf8_lossy(&data).to_string();
				} else {
					trigger_event!(client, plugin_message, &identifier, &data);
				}
			};

			let mut packet = client.read_packet(&[serverbound::configuration::CLIENT_INFORMATION])?; // Пакет Client Information

			let locale = packet.read_string()?; // for example: en_us
			let view_distance = packet.read_signed_byte()?; // client-side render distance in chunks
			let chat_mode = packet.read_varint()?; // 0: enabled, 1: commands only, 2: hidden. See Chat#Client chat mode for more information. 
			let chat_colors = packet.read_boolean()?; // this settings does nothing on client but can be used on serverside
			let displayed_skin_parts = packet.read_byte()?; // bit mask https://minecraft.wiki/w/Java_Edition_protocol#Client_Information_(configuration)
			let main_hand = packet.read_varint()?; // 0 for left and 1 for right
			let enable_text_filtering = packet.read_boolean()?; // filtering text for profanity, always false for offline mode
			let allow_server_listings = packet.read_boolean()?; // allows showing player in server listings in status
			let particle_status = packet.read_varint()?; // 0 for all, 1 for decreased, 2 for minimal

			client.set_client_info(ClientInfo {
				brand,
				locale,
				view_distance,
				chat_mode,
				chat_colors,
				displayed_skin_parts,
				main_hand,
				enable_text_filtering,
				allow_server_listings,
				particle_status,
			});

			client.write_packet(&Packet::build(
				clientbound::configuration::PLUGIN_MESSAGE,
				|p| {
					p.write_string("minecraft:brand")?;
					p.write_string(BRAND)
				},
			)?)?;

			handle_configuration_state(client.clone())?;

			client.write_packet(&Packet::empty(clientbound::configuration::FINISH))?;
			client.read_packet(&[serverbound::configuration::ACKNOWLEDGE_FINISH])?;

			client.set_state(ConnectionState::Play)?; // Мы перешли в режим Play

			// Дальше работаем с режимом игры
			handle_play_state(client)?;
		}
		_ => {
			// Тип подключения не рукопожатный
			return Err(ServerError::UnexpectedState);
		}
	}

	Ok(())
}
