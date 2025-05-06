use tokio::net::{TcpListener, TcpStream};




pub enum InetError {
	BindError,
	ConnectionClosed
}

pub struct Server {
	listener: TcpListener
}

impl Server {
	pub async fn new(addr: &str) -> Result<Self, InetError> {
		Ok(Server { listener: TcpListener::bind(addr).await.map_err(|_| InetError::BindError)? })
	}

	pub async fn accept(&self) -> TcpStream {
		loop { match self.listener.accept().await {
			Ok((stream, _)) => return stream, Err(_) => continue
		}}
	}
}