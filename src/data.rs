use std::io::{Cursor, Read, Write};

use tokio::io::{AsyncRead, AsyncReadExt};

use crate::inet::InetError;

#[derive(Debug)]
pub enum DataError {
	ReadError,
	WriteError,
	VarIntIsSoBig,
	StringDecodeError,
	Inet(InetError),
}

pub trait DataReader {
	async fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;

	async fn read_byte(&mut self) -> Result<u8, DataError> {
		Ok(self.read_bytes(1).await?[0])
	}

	async fn read_signed_byte(&mut self) -> Result<i8, DataError> {
		Ok(self.read_byte().await? as i8)
	}

	async fn read_short(&mut self) -> Result<u16, DataError> {
		Ok(u16::from_be_bytes(
			self.read_bytes(2).await?.try_into().unwrap(),
		))
	}

	async fn read_signed_short(&mut self) -> Result<i16, DataError> {
		Ok(self.read_short().await? as i16)
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

	// async fn read_packet(&mut self, threshold: Option<usize>) -> Result<Cursor<Vec<u8>>, DataError> {
	// 	let size = self.read_varint().await?;
	// 	let mut buf = self.read_bytes(size).await?;
	// }

	async fn read_string(&mut self) -> Result<String, DataError> {
		let size = self.read_varint().await?;
		let vec = self.read_bytes(size as usize).await?;
		String::from_utf8(vec).map_err(|_| DataError::StringDecodeError)
	}
}

impl<R: AsyncRead + Unpin> DataReader for R {
	async fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		let mut buf = vec![0; size];
		let mut read = 0;
		while read < size {
			match AsyncReadExt::read(self, &mut buf[read..]).await {
				Ok(0) => return Err(DataError::Inet(InetError::ConnectionClosed)),
				Ok(n) => read += n,
				Err(_) => return Err(DataError::ReadError),
			}
		}
		Ok(buf)
	}
}
