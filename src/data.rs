use std::io::{Read, Write};

use crate::inet::InetError;




pub enum DataError {
	ReadError,
	WriteError,
	Inet(InetError),
}


pub trait DataReader {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;
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