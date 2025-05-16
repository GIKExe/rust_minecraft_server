use std::io::Write;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use super::{compress, packet::Packet, DataError, Writer};



impl AsyncWriter for TcpStream {
	async fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), DataError> {
		AsyncWriteExt::write_all(self, bytes).await.or(Err(DataError::WriteError))
	}
}

pub trait AsyncWriter {
	async fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), DataError>;

	async fn write_packet(&mut self, packet: Packet, threshold: Option<usize>)
	-> Result<(), DataError> {
		let mut buf = Vec::new();

		let mut data_buf = Vec::new();
		data_buf.write_varint((packet.id() as u32) as i32)?;
		data_buf.write_bytes(packet.get_bytes())?;

		if let Some(threshold) = threshold {
			let mut packet_buf = Vec::new();

			if data_buf.len() > threshold {
				packet_buf.write_varint(data_buf.len() as i32)?;
				let compressed_data = compress(&data_buf)?;
				Write::write_all(&mut packet_buf, &compressed_data).or(Err(DataError::WriteError))?;
			} else {
				packet_buf.write_varint(0)?;
				packet_buf.write_bytes(&data_buf)?;
			}
			buf.write_varint(packet_buf.len() as i32)?;
			buf.write_bytes(&packet_buf)?;
		} else {
			buf.write_varint(data_buf.len() as i32)?;
			buf.write_bytes(&data_buf)?;
		}
		self.write_bytes(&buf).await?;
		Ok(())
	}
}