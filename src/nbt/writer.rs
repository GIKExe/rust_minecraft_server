use std::collections::HashMap;

use crate::data::{Buffer, DataError, Writer};

use super::{tags::*, Tag};



fn get_id(tag: &Tag) -> u8 {
	match tag {
		Tag::End => TAG_END,
		Tag::Byte(_) => TAG_BYTE,
		Tag::Short(_) => TAG_SHORT,
		Tag::Int(_) => TAG_INT,
		Tag::Long(_) => TAG_LONG,
		Tag::Float(_) => TAG_FLOAT,
		Tag::Double(_) => TAG_DOUBLE,
		Tag::ByteArray(_) => TAG_BYTE_ARRAY,
		Tag::String(_) => TAG_STRING,
		Tag::List(_) => TAG_LIST,
		Tag::Compound(_) => TAG_COMPOUND,
		Tag::IntArray(_) => TAG_INT_ARRAY,
		Tag::LongArray(_) => TAG_LONG_ARRAY,
	}
}

trait NbtWriterInternal {
	fn write_value(&mut self, tag: Tag) -> Result<(), DataError>;
	fn write_compound(&mut self, map: HashMap<String, Tag>) -> Result<(), DataError>;
	fn write_list(&mut self, list: Vec<Tag>) -> Result<(), DataError>;
}

impl NbtWriterInternal for Buffer {
	fn write_value(&mut self, tag: Tag) -> Result<(), DataError> {
		match tag {
			Tag::End => self.write_byte(0)?,
			Tag::Byte(value) => self.write_signed_byte(value)?,
			Tag::Short(value) => self.write_signed_short(value)?,
			Tag::Int(value) => self.write_signed_int(value)?,
			Tag::Long(value) => self.write_signed_long(value)?,
			Tag::Float(value) => self.write_float(value)?,
			Tag::Double(value) => self.write_double(value)?,
			Tag::ByteArray(value) => {
				self.write_int(value.len() as u32)?;
				let data: Vec<u8> = value.into_iter().map(|x| x as u8).collect();
				self.write_bytes(&data)?;
			}
			Tag::String(value) => {
				self.write_short(value.len() as u16)?;
				self.write_bytes(value.as_bytes())?;
			}
			Tag::List(list) => self.write_list(list)?,
			Tag::Compound(map) => self.write_compound(map)?,
			Tag::IntArray(list) => {
				self.write_int(list.len() as u32)?;
				for value in list {
					self.write_signed_int(value)?;
				}
			}
			Tag::LongArray(list) => {
				self.write_int(list.len() as u32)?;
				for value in list {
					self.write_signed_long(value)?;
				}
			}
		}
		Ok(())
	}

	fn write_compound(&mut self, map: HashMap<String, Tag>) -> Result<(), DataError> {
		for (key, tag) in map {
			self.write_byte(get_id(&tag))?;
			self.write_short(key.len() as u16)?;
			self.write_bytes(key.as_bytes())?;
			self.write_value(tag)?;
		}
		Ok(())
	}

	fn write_list(&mut self, list: Vec<Tag>) -> Result<(), DataError> {
		let type_of_list = match list.as_slice() {
			[] => 0, [first, rest @ ..] => {
				let first_id = get_id(first);
				if rest.iter().all(|tag| get_id(tag) == first_id) { first_id }
				else { return Err(DataError::NBTError) }
			}
		};
		self.write_byte(type_of_list)?;
		self.write_int(list.len() as u32)?;
		for tag in list {
			self.write_value(tag)?;
		}
		Ok(())
	}
}

pub trait NbtWriter {
	fn write_nbt(&mut self, tag: Tag, to_file: bool) -> Result<(), DataError>;
}

impl NbtWriter for Buffer {
	fn write_nbt(&mut self, tag: Tag, to_file: bool) -> Result<(), DataError> {
		let Tag::Compound(map) = tag else { return Err(DataError::NBTError); };
		self.write_byte(TAG_COMPOUND)?;
		if to_file { self.write_short(0)? };
		self.write_compound(map)?;
		Ok(())
	}
}