use std::io::Read;

use super::DataError;




pub trait Reader {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;
}

impl<R: Read> Reader for R {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		let mut buf = vec![0; size];
		match self.read_exact(&mut buf) {
			Ok(_) => Ok(buf),
			Err(_) => Err(DataError::ReadError)
		}
	}
}