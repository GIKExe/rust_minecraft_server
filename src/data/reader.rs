use std::io::{Cursor, Read};

use super::{decompress, DataError, Packet};



impl<R: Read> Reader for R {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		let mut buf = vec![0; size];
		match self.read_exact(&mut buf) {
			Ok(_) => Ok(buf),
			Err(_) => Err(DataError::ReadError)
		}
	}
}

pub trait Reader {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError>;

	fn read_byte(&mut self) -> Result<u8, DataError> {
		Ok(self.read_bytes(1)?[0])
	}

	fn read_signed_byte(&mut self) -> Result<i8, DataError> {
		Ok(self.read_byte()? as i8)
	}

	fn read_varint_size(&mut self) -> Result<(i32, usize), DataError> {
		let mut value = 0;
		let mut position = 0;
		loop {
			let byte = self.read_byte()?;
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

	fn read_varint(&mut self) -> Result<i32, DataError> {
		Ok(self.read_varint_size()?.0)
	}

	fn read_packet(&mut self, threshold: Option<usize>)
	-> Result<Packet, DataError> {
		let mut data: Vec<u8>;
		let packet_lenght = self.read_varint()? as usize;
		if threshold.is_some() {
			let data_lenght = self.read_varint_size()?;
			data = self.read_bytes(packet_lenght - data_lenght.1)?;
			if data_lenght.0 != 0 { data = decompress(&data)?; }
		} else {
			data = self.read_bytes(packet_lenght)?;
		}
		let mut cursor = Cursor::new(data);
		let id = cursor.read_varint()?;
		Ok(Packet::new(id as u8, cursor))
	}

	fn read_short(&mut self) -> Result<u16, DataError> {
		Ok(u16::from_be_bytes(
			self.read_bytes(2)?.try_into().unwrap()
		))
	}

	fn read_signed_short(&mut self) -> Result<i16, DataError> {
		Ok(self.read_short()? as i16)
	}

	fn read_string(&mut self) -> Result<String, DataError> {
		let size = self.read_varint()?;
		let vec = self.read_bytes(size as usize)?;
		String::from_utf8(vec).or( Err(DataError::StringDecodeError))
	}

	fn read_long(&mut self) -> Result<u64, DataError> {
		Ok(u64::from_be_bytes(
			self.read_bytes(8)?.try_into().unwrap()
		))
	}

	fn read_signed_long(&mut self) -> Result<i64, DataError> {
		Ok(self.read_long()? as i64)
	}

	fn read_uuid(&mut self) -> Result<u128, DataError> {
		Ok(u128::from_be_bytes(
			self.read_bytes(16)?.try_into().unwrap()
		))
	}
}