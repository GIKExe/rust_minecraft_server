use std::io::{Read, Write};

#[derive(Debug)]
pub enum ProtocolError {
	ReadError,
	ConnectionClosed,
	WriteError
}

pub trait DataReader {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, ProtocolError>;

	fn read_byte(&mut self) -> Result<u8, ProtocolError> {
		Ok(self.read_bytes(1)?[0])
	}
}

impl <R: Read> DataReader for R {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, ProtocolError> {
		let mut buf = vec![0; size];
		match self.read(&mut buf) {
			Ok(n) => if n == 0 {
				return Err(ProtocolError::ConnectionClosed);
			} else if n < size {
				buf.truncate(n);
				buf.append(&mut self.read_bytes(size - n)?);
			}, Err(_) => return Err(ProtocolError::ReadError)
		}; Ok(buf)
	}
}



pub trait DataWriter {
	fn write_bytes(&mut self, buf: &mut Vec<u8>) -> Result<(), ProtocolError>;

	fn write_byte(&mut self, byte: u8) -> Result<(), ProtocolError> {
		self.write_bytes(&mut vec![byte])
	}
}

impl <W: Write> DataWriter for W {
	fn write_bytes(&mut self, buf: &mut Vec<u8>) -> Result<(), ProtocolError> {
		self.write_all(buf).map_err(|_| ProtocolError::WriteError)
	}
}