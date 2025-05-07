use std::io::{Read, Write};

use tokio::io::{AsyncRead, AsyncReadExt};

use crate::inet::InetError;




pub enum DataError {
	ReadError,
	WriteError,
	VarIntIsSoBig,
	StringDecodeError,
	Inet(InetError),
}

pub trait ReadLike {
	type Output;
	fn _read_bytes(self, size: usize) -> Self::Output;
}

impl <R: Read> ReadLike for &mut R {
	type Output = Result<Vec<u8>, DataError>;

	fn _read_bytes(self, size: usize) -> Self::Output {
		let mut buf = vec![0; size];
		let mut read = 0;
		while read < size {
			match self.read(&mut buf[read..]) {
				Ok(0) => return Err(DataError::Inet(InetError::ConnectionClosed)),
				Ok(n) => read+=n,
				Err(_) => return Err(DataError::ReadError)
			}
		}; Ok(buf)
	}
}

#[cfg(feature = "async")]
#[async_trait]
impl <R: AsyncRead + Unpin + Send> ReadLike for &mut R {
	type Output = Result<Vec<u8>, DataError>;

	async fn _read_bytes(self, size: usize) -> Self::Output {
		let mut buf = vec![0; size];
		let mut read = 0;
		while read < size {
			match AsyncReadExt::read(&mut self, &mut buf[read..]).await {
				Ok(0) => return Err(DataError::Inet(InetError::ConnectionClosed)),
				Ok(n) => read += n,
				Err(_) => return Err(DataError::ReadError),
			}
		}
		Ok(buf)
	}
}

#[cfg_attr(feature = "async", async_trait)]
pub trait DataReader {
	 #[cfg(not(feature = "async"))]
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;

	#[cfg(feature = "async")]
	async fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;

	fn read_byte(&mut self) -> Result<u8, DataError> {
		Ok(self.read_bytes(1)?[0])
	}

	fn read_signed_byte(&mut self) -> Result<i8, DataError> {
		Ok(self.read_byte()? as i8)
	}

	fn read_short(&mut self) -> Result<u16, DataError> {
		Ok(u16::from_be_bytes(self.read_bytes(2)?.try_into().unwrap()))
	}

	fn read_signed_short(&mut self) -> Result<i16, DataError> {
		Ok(self.read_short()? as i16)
	}

	fn read_varint(&mut self) -> Result<i32, DataError> {
		let mut value = 0;
		let mut position = 0;
		loop {
			let byte = self.read_byte()?;
			value |= ((byte & 0x7F) << position) as i32;
			if (byte & 0x80) == 0 {return Ok(value)};
			position += 7;
			if position >= 32 {return Err(DataError::VarIntIsSoBig)};
		}
	}

	fn read_string(&mut self) -> Result<String, DataError> {
		let size = self.read_varint()?;
		let vec = self.read_bytes(size as usize)?;
		String::from_utf8(vec).map_err(|_| DataError::StringDecodeError)
	}
}

impl <R: Read> DataReader for R {
	#[cfg(not(feature = "async"))]
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		self._read_bytes(size)
	}
}

#[cfg(feature = "async")]
#[async_trait]
impl<R: AsyncRead + Unpin + Send> DataReader for R {
	async fn _read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		self.read_bytes(size).await
	}
}



pub trait DataWriter {
	fn write_bytes(&mut self, buf: Vec<u8>) -> Result<(), DataError>;
}

impl <W: Write> DataWriter for W {
	fn write_bytes(&mut self, buf: Vec<u8>) -> Result<(), DataError> {
		self.write_all(&buf).map_err(|_| DataError::WriteError)
	}
}