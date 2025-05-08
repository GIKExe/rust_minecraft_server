use std::io::{Cursor, ErrorKind};

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::inet::InetError;

use super::{decompress, packet::Packet, DataError, Reader};



impl AsyncReader for TcpStream {
	async fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		let mut buf = vec![0; size];
		match AsyncReadExt::read_exact(self, &mut buf).await {
			Ok(_) => Ok(buf),
			Err(e) => match e.kind() {
				ErrorKind::UnexpectedEof | ErrorKind::BrokenPipe | ErrorKind::ConnectionReset => {
					Err(DataError::Inet(InetError::ConnectionClosed))
				}
				_ => {
					Err(DataError::ReadError)
				},
			}
		}
	}
}

pub trait AsyncReader {
	async fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;

	async fn read_byte(&mut self) -> Result<u8, DataError> {
		Ok(self.read_bytes(1).await?[0])
	}

	async fn read_signed_byte(&mut self) -> Result<i8, DataError> {
		Ok(self.read_byte().await? as i8)
	}

	async fn read_varint_size(&mut self) -> Result<(i32, usize), DataError> {
		let mut value = 0;
		let mut position = 0;
		loop {
			let byte = self.read_byte().await?;
			value |= ((byte & 0x7F) as i32) << (position * 7);
			if (byte & 0x80) == 0 {
				return Ok((value, position as usize));
			};
			position += 1;
			if position >= 5 {
				return Err(DataError::VarIntIsSoBig);
			};
		}
	}

	async fn read_varint(&mut self) -> Result<i32, DataError> {
		Ok(self.read_varint_size().await?.0)
	}

	async fn read_packet(&mut self, threshold: Option<usize>)
	-> Result<Packet, DataError> {
		let mut data: Vec<u8>;
		let packet_lenght = self.read_varint().await? as usize;
		if threshold.is_some() {
			let data_lenght = self.read_varint_size().await?;
			data = self.read_bytes(packet_lenght - data_lenght.1).await?;
			if data_lenght.0 != 0 { data = decompress(&data)?; }
		} else {
			data = self.read_bytes(packet_lenght).await?;
		}
		let mut cursor = Cursor::new(data);
		let id = cursor.read_varint()?;
		Ok(Packet::new(id as u8, cursor))
	}

	async fn read_short(&mut self) -> Result<u16, DataError> {
		Ok(u16::from_be_bytes(
			self.read_bytes(2).await?.try_into().unwrap()
		))
	}

	async fn read_signed_short(&mut self) -> Result<i16, DataError> {
		Ok(self.read_short().await? as i16)
	}

	async fn read_string(&mut self) -> Result<String, DataError> {
		let size = self.read_varint().await?;
		let vec = self.read_bytes(size as usize).await?;
		String::from_utf8(vec).or( Err(DataError::StringDecodeError))
	}

	async fn read_long(&mut self) -> Result<u64, DataError> {
		Ok(u64::from_be_bytes(
			self.read_bytes(8).await?.try_into().unwrap()
		))
	}

	async fn read_signed_long(&mut self) -> Result<i64, DataError> {
		Ok(self.read_long().await? as i64)
	}

	async fn read_uuid(&mut self) -> Result<u128, DataError> {
		Ok(u128::from_be_bytes(
			self.read_bytes(16).await?.try_into().unwrap()
		))
	}
}