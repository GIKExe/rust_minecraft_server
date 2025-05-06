use std::net::{TcpListener, TcpStream};



pub enum InetError {
	BindError,
	ConnectionClosed
}



pub struct Server {
	host: String,
	port: u16,
	listener: TcpListener
}

impl Server {
	pub fn new(host: &str, port: u16) -> Result<Self, InetError> {
		Ok(Server {
			host: host.to_string(),
			port,
			listener: TcpListener::bind(format!("{host}:{port}")).map_err(|_| InetError::BindError)?
		})
	}

	pub fn accept(&self) -> TcpStream {
		loop {
			match self.listener.accept() {
				Ok((stream, _)) => return stream,
				Err(_) => continue
			}
		}
	}
}