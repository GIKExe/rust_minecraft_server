use std::{net::SocketAddr, sync::Arc};

use dashmap::DashMap;
use itertools::Itertools;
use uuid::Uuid;

use super::{
	config::Config,
	event::{Listener, PacketHandler},
	player::context::ClientContext,
};

// Контекст сервера
// Должен быть обернут в Arc для передачи между потоками
pub struct ServerContext {
	pub config: Arc<Config>,
	pub clients: DashMap<SocketAddr, Arc<ClientContext>>,
	listeners: Vec<Box<dyn Listener>>,
	handlers: Vec<Box<dyn PacketHandler>>,
}

impl ServerContext {
	pub fn new(config: Arc<Config>) -> ServerContext {
		ServerContext {
			config,
			listeners: Vec::new(),
			handlers: Vec::new(),
			clients: DashMap::new(),
		}
	}

	pub fn get_player_by_uuid(self: &Arc<Self>, uuid: Uuid) -> Option<Arc<ClientContext>> {
		self
			.clients
			.iter()
			.find(|o| {
				let info = o.player_info();
				if let Some(info) = info {
					info.uuid == uuid
				} else {
					false
				}
			})
			.map(|o| o.clone())
	}

	pub fn get_player_by_name(self: &Arc<Self>, name: &str) -> Option<Arc<ClientContext>> {
		self
			.clients
			.iter()
			.find(|o| {
				let info = o.player_info();
				if let Some(info) = info {
					info.name == name
				} else {
					false
				}
			})
			.map(|o| o.clone())
	}

	pub fn players(self: &Arc<Self>) -> Vec<Arc<ClientContext>> {
		self
			.clients
			.iter()
			.filter(|o| o.player_info().is_some())
			.map(|o| o.clone())
			.collect()
	}

	pub fn add_packet_handler(&mut self, handler: Box<dyn PacketHandler>) {
		self.handlers.push(handler);
	}

	pub fn add_listener(&mut self, listener: Box<dyn Listener>) {
		self.listeners.push(listener);
	}

	pub fn packet_handlers<F, K>(self: &Arc<Self>, sort_by: F) -> Vec<&Box<dyn PacketHandler>>
	where
		K: Ord,
		F: FnMut(&&Box<dyn PacketHandler>) -> K,
	{
		self.handlers.iter().sorted_by_key(sort_by).collect_vec()
	}

	pub fn listeners<F, K>(self: &Arc<Self>, sort_by: F) -> Vec<&Box<dyn Listener>>
	where
		K: Ord,
		F: FnMut(&&Box<dyn Listener>) -> K,
	{
		self.listeners.iter().sorted_by_key(sort_by).collect_vec()
	}
}
