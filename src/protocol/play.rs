use std::{
	io::Cursor,
	sync::Arc,
	thread,
	time::{Duration, SystemTime, UNIX_EPOCH},
};

use rust_mc_proto::{DataReader, DataWriter, Packet, read_packet};

use crate::{
	ServerError,
	data::{ReadWriteNBT, text_component::TextComponent},
	player::context::ClientContext,
};

use super::id::*;

pub fn send_update_tags(client: Arc<ClientContext>) -> Result<(), ServerError> {
	// TODO: rewrite this hardcode bullshit

	client.write_packet(&Packet::from_bytes(
		clientbound::configuration::UPDATE_TAGS,
		include_bytes!("update-tags.bin"),
	))
}

pub fn send_registry_data(client: Arc<ClientContext>) -> Result<(), ServerError> {
	// TODO: rewrite this hardcode bullshit

	let mut registry_data = Cursor::new(include_bytes!("registry-data.bin"));

	while let Ok(mut packet) = read_packet(&mut registry_data, None) {
		packet.set_id(clientbound::configuration::REGISTRY_DATA);
		client.write_packet(&packet)?;
	}

	Ok(())
}

// Добавки в Configuration стейт чтобы все работало
pub fn handle_configuration_state(
	client: Arc<ClientContext>, // Контекст клиента
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::configuration::FEATURE_FLAGS);
	packet.write_varint(1)?;
	packet.write_string("minecraft:vanilla")?;
	client.write_packet(&packet)?;

	let mut packet = Packet::empty(clientbound::configuration::KNOWN_PACKS);
	packet.write_varint(1)?;
	packet.write_string("minecraft")?;
	packet.write_string("core")?;
	packet.write_string("1.21.5")?;
	client.write_packet(&packet)?;

	client.read_packet(&[serverbound::configuration::KNOWN_PACKS])?;

	send_registry_data(client.clone())?;
	send_update_tags(client.clone())
}

pub fn send_login(client: Arc<ClientContext>) -> Result<(), ServerError> {
	// Отправка пакета Login
	let mut packet = Packet::empty(clientbound::play::LOGIN);

	packet.write_int(0)?; // Entity ID
	packet.write_boolean(false)?; // Is hardcore
	packet.write_varint(4)?; // Dimension Names
	packet.write_string("minecraft:overworld")?;
	packet.write_string("minecraft:nether")?;
	packet.write_string("minecraft:the_end")?;
	packet.write_string("minecraft:overworld_caves")?;
	packet.write_varint(0)?; // Max Players
	packet.write_varint(8)?; // View Distance
	packet.write_varint(5)?; // Simulation Distance
	packet.write_boolean(false)?; // Reduced Debug Info
	packet.write_boolean(true)?; // Enable respawn screen
	packet.write_boolean(false)?; // Do limited crafting

	packet.write_varint(0)?; // Dimension Type
	packet.write_string("minecraft:overworld")?; // Dimension Name
	packet.write_long(0x0f38f26ad09c3e20)?; // Hashed seed
	packet.write_byte(0)?; // Game mode
	packet.write_signed_byte(-1)?; // Previous Game mode
	packet.write_boolean(false)?; // Is Debug
	packet.write_boolean(true)?; // Is Flat
	packet.write_boolean(false)?; // Has death location
	packet.write_varint(20)?; // Portal cooldown
	packet.write_varint(60)?; // Sea level

	packet.write_boolean(false)?; // Enforces Secure Chat

	client.write_packet(&packet)
}

pub fn send_game_event(
	client: Arc<ClientContext>,
	event: u8,
	value: f32,
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::GAME_EVENT);

	packet.write_byte(event)?;
	packet.write_float(value)?;

	client.write_packet(&packet)
}

pub fn sync_player_pos(
	client: Arc<ClientContext>,
	x: f64,
	y: f64,
	z: f64,
	vel_x: f64,
	vel_y: f64,
	vel_z: f64,
	yaw: f32,
	pitch: f32,
	flags: i32,
) -> Result<(), ServerError> {
	let timestamp = (SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis()
		& 0xFFFFFFFF) as i32;

	let mut packet = Packet::empty(clientbound::play::SYNCHRONIZE_PLAYER_POSITION);

	packet.write_varint(timestamp)?;
	packet.write_double(x)?;
	packet.write_double(y)?;
	packet.write_double(z)?;
	packet.write_double(vel_x)?;
	packet.write_double(vel_y)?;
	packet.write_double(vel_z)?;
	packet.write_float(yaw)?;
	packet.write_float(pitch)?;
	packet.write_int(flags)?;

	client.write_packet(&packet)?;

	Ok(())
}

pub fn set_center_chunk(client: Arc<ClientContext>, x: i32, z: i32) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::SET_CENTER_CHUNK);

	packet.write_varint(x)?;
	packet.write_varint(z)?;

	client.write_packet(&packet)
}

pub fn send_example_chunk(client: Arc<ClientContext>, x: i32, z: i32) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::CHUNK_DATA_AND_UPDATE_LIGHT);

	packet.write_int(x)?;
	packet.write_int(z)?;

	// heightmap

	packet.write_varint(1)?; // heightmaps count
	packet.write_varint(0)?; // MOTION_BLOCKING
	packet.write_varint(256)?; // Length of the following long array (16 * 16 = 256)
	for _ in 0..256 {
		packet.write_long(0)?; // height - 0
	}

	// sending chunk data

	let mut chunk_data = Vec::new();

	// we want to fill the area from -64 to 0, so it will be 4 chunk sections

	for _ in 0..4 {
		chunk_data.write_short(4096)?; // non-air blocks count, 16 * 16 * 16 = 4096 stone blocks

		// blocks paletted container
		chunk_data.write_byte(0)?; // Bits Per Entry, use Single valued palette format
		chunk_data.write_varint(10)?; // block state id in the registry (1 for stone)

		// biomes palleted container
		chunk_data.write_byte(0)?; // Bits Per Entry, use Single valued palette format
		chunk_data.write_varint(27)?; // biome id in the registry
	}

	// air chunk sections

	for _ in 0..20 {
		chunk_data.write_short(0)?; // non-air blocks count, 0

		// blocks paletted container
		chunk_data.write_byte(0)?; // Bits Per Entry, use Single valued palette format
		chunk_data.write_varint(0)?; // block state id in the registry (0 for air)

		// biomes palleted container
		chunk_data.write_byte(0)?; // Bits Per Entry, use Single valued palette format
		chunk_data.write_varint(27)?; // biome id in the registry
	}

	packet.write_usize_varint(chunk_data.len())?;
	packet.write_bytes(&chunk_data)?;

	packet.write_byte(0)?;

	// light data

	packet.write_byte(0)?;
	packet.write_byte(0)?;
	packet.write_byte(0)?;
	packet.write_byte(0)?;
	packet.write_byte(0)?;
	packet.write_byte(0)?;

	client.write_packet(&packet)?;

	Ok(())
}

pub fn send_keep_alive(client: Arc<ClientContext>) -> Result<(), ServerError> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs() as i64;

	let mut packet = Packet::empty(clientbound::play::KEEP_ALIVE);
	packet.write_long(timestamp)?;
	client.write_packet(&packet)?;

	let mut packet = client.read_packet(&[serverbound::play::KEEP_ALIVE])?;
	let timestamp2 = packet.read_long()?;
	if timestamp2 != timestamp {
		// Послать клиента нахуй
		Err(ServerError::WrongPacket)
	} else {
		Ok(())
	}
}

pub fn send_system_message(
	client: Arc<ClientContext>,
	message: TextComponent,
	is_action_bar: bool,
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::SYSTEM_CHAT_MESSAGE);
	packet.write_nbt(&message)?;
	packet.write_boolean(is_action_bar)?;
	client.write_packet(&packet)
}

pub fn send_example_chunks_in_distance(
	client: Arc<ClientContext>,
	chunks: &mut Vec<(i32, i32)>,
	distance: i32,
	center: (i32, i32),
) -> Result<(), ServerError> {
	for x in -distance + center.0..=distance + center.0 {
		for z in -distance + center.1..=distance + center.1 {
			if !chunks.contains(&(x, z)) {
				send_example_chunk(client.clone(), x as i32, z as i32)?;
				chunks.push((x, z));
			}
		}
	}

	Ok(())
}

// Отдельная функция для работы с самой игрой
pub fn handle_play_state(
	client: Arc<ClientContext>, // Контекст клиента
) -> Result<(), ServerError> {
	thread::spawn({
		let client = client.clone();

		move || {
			let _ = client.run_read_loop();
			client.close();
		}
	});

	send_login(client.clone())?;
	sync_player_pos(client.clone(), 8.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0)?;
	send_game_event(client.clone(), 13, 0.0)?; // 13 - Start waiting for level chunks
	set_center_chunk(client.clone(), 0, 0)?;

	let mut chunks = Vec::new();

	let view_distance = client.client_info().unwrap().view_distance as i32;

	send_example_chunks_in_distance(client.clone(), &mut chunks, view_distance, (0, 0))?;

	thread::spawn({
		let client = client.clone();

		move || -> Result<(), ServerError> {
			while client.is_alive() {
				let mut packet = client.read_any_packet()?;

				match packet.id() {
					serverbound::play::SET_PLAYER_POSITION => {
						let x = packet.read_double()?;
						let y = packet.read_double()?;
						let z = packet.read_double()?;
						let _ = packet.read_byte()?; // flags

						client.set_position((x, y, z));
					}
					serverbound::play::SET_PLAYER_POSITION_AND_ROTATION => {
						let x = packet.read_double()?;
						let y = packet.read_double()?;
						let z = packet.read_double()?;
						let yaw = packet.read_float()?;
						let pitch = packet.read_float()?;
						let _ = packet.read_byte()?; // flags

						client.set_position((x, y, z));
						client.set_rotation((yaw, pitch));
					}
					serverbound::play::SET_PLAYER_ROTATION => {
						let yaw = packet.read_float()?;
						let pitch = packet.read_float()?;
						let _ = packet.read_byte()?; // flags

						client.set_rotation((yaw, pitch));
					}
					_ => {
						client.push_packet_back(packet);
					}
				}
			}

			Ok(())
		}
	});

	let mut ticks_alive = 0u64;

	while client.is_alive() {
		if ticks_alive % 200 == 0 {
			// 10 secs timer
			send_keep_alive(client.clone())?;
		}

		if ticks_alive % 20 == 0 {
			// 1 sec timer
			let (x, y, z) = client.position();

			let (chunk_x, chunk_z) = ((x / 16.0) as i64, (z / 16.0) as i64);
			let (chunk_x, chunk_z) = (chunk_x as i32, chunk_z as i32);

			set_center_chunk(client.clone(), chunk_x, chunk_z)?;
			send_example_chunks_in_distance(
				client.clone(),
				&mut chunks,
				view_distance,
				(chunk_x, chunk_z),
			)?;

			send_system_message(
				client.clone(),
				TextComponent::rainbow(format!("Pos: {} {} {}", x as i64, y as i64, z as i64)),
				false,
			)?;
		}

		send_system_message(
			client.clone(),
			TextComponent::rainbow(format!("Ticks alive: {}", ticks_alive)),
			true,
		)?;

		thread::sleep(Duration::from_millis(50)); // 1 tick
		ticks_alive += 1;
	}

	Ok(())
}
