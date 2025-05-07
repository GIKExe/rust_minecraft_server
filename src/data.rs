use std::io::{Read, Write};

use crate::inet::InetError;




pub enum DataError {
	ReadError,
	WriteError,
	VarIntIsSoBig,
	StringDecodeError,
	Inet(InetError),
}


pub trait DataReader {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;

	fn read_byte(&mut self) -> Result<u8, DataError> {
		Ok(self.read_bytes(1)?[0])
	}

	fn read_byte_signed(&mut self) -> Result<i8, DataError> {
		Ok(self.read_byte()? as i8)
	}

	fn read_short(&mut self) -> Result<u16, DataError> {
		Ok((self.read_byte()? as u16) + ((self.read_byte()? as u16) << 8))
	}

	fn read_short_signed(&mut self) -> Result<i16, DataError> {
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
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		let mut buf = vec![0; size];
		match self.read(&mut buf) {
			Err(_) => return Err(DataError::ReadError),
			Ok(n) => if n == 0 {
				return Err(DataError::Inet(InetError::ConnectionClosed));
			} else if n < size {
				buf.truncate(n);
				buf.append(&mut self.read_bytes(size - n)?);
			}
		}; Ok(buf)
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