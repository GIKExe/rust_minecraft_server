use std::{collections::HashMap, io::Seek};

use crate::data::{Buffer, DataError, Reader};

use super::{tags::*, Tag};

trait NbtReaderInternal {
	fn read_value(&mut self, t: u8) -> Result<Tag, DataError>;
	fn read_compound(&mut self) -> Result<Tag, DataError>;
	fn read_list(&mut self) -> Result<Tag, DataError>;
}

impl NbtReaderInternal for Buffer {
	fn read_value(&mut self, t: u8) -> Result<Tag, DataError> {
		match t {
			TAG_END => { self.read_byte()?; Ok(Tag::End) }
			TAG_BYTE => { Ok(Tag::Byte(self.read_signed_byte()?)) }
			TAG_SHORT => { Ok(Tag::Short(self.read_signed_short()?)) }
			TAG_INT => { Ok(Tag::Int(self.read_signed_int()?)) }
			TAG_LONG => { Ok(Tag::Long(self.read_signed_long()?)) }
			TAG_FLOAT => { Ok(Tag::Float(self.read_float()?)) }
			TAG_DOUBLE => { Ok(Tag::Double(self.read_double()?)) }
			TAG_BYTE_ARRAY => {
				let size = self.read_int()? as usize;
				Ok(Tag::ByteArray(self.read_bytes(size)?.into_iter().map(|x| x as i8).collect()))
			}
			TAG_STRING => {
				let size = self.read_short()? as usize;
				let data = self.read_bytes(size)?;
				Ok(Tag::String(String::from_utf8_lossy(&data).to_string()))
			}
			TAG_LIST => { self.read_list() }
			TAG_COMPOUND => { self.read_compound() }
			TAG_INT_ARRAY => {
				let size = self.read_int()? as usize;
				let mut data = Vec::new();
				for _ in 0..size {
					data.push(self.read_signed_int()?);
				}
				Ok(Tag::IntArray(data))
			}
			TAG_LONG_ARRAY => {
				let size = self.read_int()? as usize;
				let mut data = Vec::new();
				for _ in 0..size {
					data.push(self.read_signed_long()?);
				}
				Ok(Tag::LongArray(data))
			}
			other => {
				println!("Неверный тэг: {other}");
				Err(DataError::NBTError)
			}
		}
	}

	fn read_compound(&mut self) -> Result<Tag, DataError> {
		let mut map = HashMap::new();
		loop {
			let type_of_value = self.read_byte()?;
			if type_of_value == TAG_END { break; }
			let size = self.read_short()? as usize;
			let key = String::from_utf8_lossy(&self.read_bytes(size)?).to_string();
			map.insert(key, self.read_value(type_of_value)?);
		}
		Ok(Tag::Compound(map))
	}

	fn read_list(&mut self) -> Result<Tag, DataError> {
		let type_of_list = self.read_byte()?;
		let size_of_list = self.read_signed_int()?;
		let mut list = Vec::new();
		if size_of_list <= 0 { return Ok(Tag::List(list)); };
		for _ in 0..size_of_list {
			list.push(self.read_value(type_of_list)?);
		}
		Ok(Tag::List(list))
	}
}

pub trait NbtReader {
	fn read_nbt(&mut self) -> Result<Tag, DataError>;
}

impl NbtReader for Buffer {
	fn read_nbt(&mut self) -> Result<Tag, DataError> {

		if self.read_byte()? != TAG_COMPOUND {
			return Err(DataError::NBTError);
		}

		if self.read_short()? != 0 {
			let _ = self.seek_relative(-2);
		}

		self.read_compound()
	}
}