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

	async fn write_byte(&mut self, value: u8) -> Result<(), DataError> {
		self.write_bytes(&[value]).await
	}

	async fn write_signed_byte(&mut self, value: i8) -> Result<(), DataError> {
		self.write_byte(value as u8).await
	}

	async fn write_varint_size(&mut self, value: i32) -> Result<usize, DataError> {
		let mut _value = value as u32;
		let mut position = 0;
		loop {
			let mut byte = (_value & 127) as u8;
			position += 1; _value >>= 7;
			if _value != 0 { byte += 128; };
			self.write_byte(byte).await?;
			if _value == 0 { return Ok(position) }
		}
	}

	async fn write_varint(&mut self, value: i32) -> Result<(), DataError> {
		self.write_varint_size(value).await?; Ok(())
	}

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

	async fn write_short(&mut self, value: u16) -> Result<(), DataError> {
		self.write_bytes(&value.to_be_bytes()).await
	}

	async fn write_signed_short(&mut self, value: i16) -> Result<(), DataError> {
		self.write_short(value as u16).await
	}

	async fn write_string(&mut self, value: &str) -> Result<(), DataError> {
		self.write_varint(value.len() as i32).await?;
		self.write_bytes(value.as_bytes()).await?;
		Ok(())
	}

	async fn write_long(&mut self, value: u64) -> Result<(), DataError> {
		self.write_bytes(&value.to_be_bytes()).await
	}

	async fn write_signed_long(&mut self, value: i64) -> Result<(), DataError> {
		self.write_long(value as u64).await
	}

	async fn write_uuid(&mut self, value: u128) -> Result<(), DataError> {
		self.write_bytes(&value.to_be_bytes()).await
	}
}