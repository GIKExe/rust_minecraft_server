use std::{error::Error, fmt::Display, net::TcpListener, sync::Arc, thread, time::Duration};

use config::Config;
use context::ServerContext;
use ignore_result::Ignore;
use log::{error, info};
use player::context::ClientContext;
use protocol::handler::handle_connection;
use rust_mc_proto::{MinecraftConnection, ProtocolError};

pub mod config;
pub mod context;
pub mod data;
pub mod event;
pub mod player;
pub mod protocol;

// Ошибки сервера
#[derive(Debug)]
pub enum ServerError {
	UnexpectedPacket(u8),    // Неожиданный пакет
	WrongPacket,             // Пакет поломан, неверные данные
	Protocol(ProtocolError), // Ошибка в протоколе при работе с rust_mc_proto
	ConnectionClosed, // Соединение закрыто, единственная ошибка которая не логируется у handle_connection
	SerTextComponent, // Ошибка при сериализации текст-компонента
	DeTextComponent,  // Ошибка при десериализации текст-компонента
	SerNbt,           // Ошибка при сериализации nbt
	DeNbt,            // Ошибка при десериализации nbt
	UnexpectedState, // Указывает на то что этот пакет не может быть отправлен в данном режиме (в основном через ProtocolHelper)
	Other(String), // Другая ошибка, либо очень специфичная, либо хз, лучше не использовать и создавать новое поле ошибки
}

impl Display for ServerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("{:?}", self))
	}
}

impl Error for ServerError {}

// Делаем чтобы ProtocolError мог переделываться в наш ServerError
impl From<ProtocolError> for ServerError {
	fn from(error: ProtocolError) -> ServerError {
		match error {
			// Если просто закрыто соединение, переделываем в нашу ошибку этого
			ProtocolError::ConnectionClosedError => ServerError::ConnectionClosed,
			// Все остальное просто засовываем в обертку
			error => ServerError::Protocol(error),
		}
	}
}

pub fn start_server(server: Arc<ServerContext>) {
	// Биндим сервер где надо
	let Ok(listener) = TcpListener::bind(&server.config.bind.host) else {
		error!(
			"Не удалось забиндить сервер на {}",
			&server.config.bind.host
		);
		return;
	};

	info!("Сервер запущен на {}", &server.config.bind.host);

	while let Ok((stream, addr)) = listener.accept() {
		let server = server.clone();

		thread::spawn(move || {
			info!("Подключение: {}", addr);

			// Установка таймаутов на чтение и запись
			// По умолчанию пусть будет 5 секунд, надо будет сделать настройку через конфиг
			stream
				.set_read_timeout(Some(Duration::from_secs(server.config.bind.timeout)))
				.ignore();
			stream
				.set_write_timeout(Some(Duration::from_secs(server.config.bind.timeout)))
				.ignore();

			// Оборачиваем стрим в майнкрафт конекшн лично для нашего удовольствия
			let conn = MinecraftConnection::new(stream);

			// Создаем контекст клиента
			// Передавется во все листенеры и хандлеры чтобы определять именно этот клиент
			let client = Arc::new(ClientContext::new(server.clone(), conn));

			// Добавляем клиента в список клиентов сервера
			// Используем адрес как ключ, врятли ipv4 будет нам врать
			server.clients.insert(client.addr, client.clone());

			// Обработка подключения
			// Если ошибка -> выводим
			match handle_connection(client.clone()) {
				Ok(_) => {}
				Err(ServerError::ConnectionClosed) => {}
				Err(error) => {
					error!("Ошибка подключения: {error:?}");
				}
			};

			// Удаляем клиента из списка клиентов
			server.clients.remove(&client.addr);

			info!("Отключение: {}", addr);
		});
	}
}

// server start helper
// pub struct Server {
// 	context: Arc<ServerContext>,
// }

// impl Server {
// 	pub fn new(context: ServerContext) -> Self {
// 		Self {
// 			context: Arc::new(context),
// 		}
// 	}

// 	pub fn context(&self) -> &ServerContext {
// 		&self.context
// 	}

// 	pub fn start(&self) {
// 		start_server(self.context.clone());
// 	}
// }

// impl Default for Server {
// 	fn default() -> Self {
// 		Self {
// 			context: Arc::new(ServerContext::new(Arc::new(Config::default()))),
// 		}
// 	}
// }
