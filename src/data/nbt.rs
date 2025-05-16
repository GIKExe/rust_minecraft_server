use std::io::Cursor;

use super::{DataError, Reader};




const TAG_END:        u8 =  0;
const TAG_BYTE:       u8 =  1;
const TAG_SHORT:      u8 =  2;
const TAG_INT:        u8 =  3;
const TAG_LONG:       u8 =  4;
const TAG_FLOAT:      u8 =  5;
const TAG_DOUBLE:     u8 =  6;
const TAG_BYTE_ARRAY: u8 =  7;
const TAG_STRING:     u8 =  8;
const TAG_LIST:       u8 =  9;
const TAG_COMPOUND:   u8 = 10;
const TAG_INT_ARRAY:  u8 = 11;
const TAG_LONG_ARRAY: u8 = 12;

pub enum TAG {
	END,
	COMPOUND(Vec<NBT>),
	STRING(String),
	BYTE(i8),
	SHORT(i16),
	INT(i32),
	LONG(i64),
	FLOAT(f32),
	DOUBLE(f64),
	LIST(Vec<NBT>),
	BYTE_ARRAY(Vec<i8>),
	INT_ARRAY(Vec<i32>),
	LONG_ARRAY(Vec<i64>),
}

struct NBT {
	key: String,
	value: TAG,
}

impl NBT {
	fn new(key: &str, value: TAG) -> Self {
		Self { key: key.to_string(), value }
	}
}

pub trait NBT_Reader {
	fn read_nbt(&mut self, root: bool, inet: bool, byte: Option<u8>) -> Result<NBT, DataError>;
}

impl NBT_Reader for dyn Reader {
	fn read_nbt(&mut self, root: bool, inet: bool, byte: Option<u8>) -> Result<NBT, DataError> {
		let mut byte = match byte { Some(v) => v, None => self.read_byte()? };
		let key: String;

		if root & inet {
			key = "".to_string();
			if byte != TAG_COMPOUND {
				return Err(DataError::NBTError);
			}
		} else {
			let size = self.read_short()? as usize;
			let buf = self.read_bytes(size)?;
			key = String::from_utf8_lossy(&buf).to_string();
		}

		match byte {
			TAG_BYTE => { Ok(NBT::new(&key, TAG::BYTE(self.read_signed_byte()?))) },
			TAG_SHORT => { Ok(NBT::new(&key, TAG::SHORT(self.read_signed_short()?))) },
			TAG_INT => { Ok(NBT::new(&key, TAG::INT(self.read_signed_int()?))) },
			TAG_LONG => { Ok(NBT::new(&key, TAG::LONG(self.read_signed_long()?))) },
			TAG_FLOAT => { Ok(NBT::new(&key, TAG::FLOAT(self.read_float()?))) },
			TAG_DOUBLE => { Ok(NBT::new(&key, TAG::DOUBLE(self.read_double()?))) },

			TAG_STRING => {
				let size = self.read_short()? as usize;
				let buf = self.read_bytes(size)?;
				Ok(NBT::new(&key, TAG::STRING(String::from_utf8_lossy(&buf).to_string())))
			},

			TAG_BYTE_ARRAY => {
				let size = self.read_signed_int()? as usize;
				let buf =  self.read_bytes(size)?.into_iter().map(|x| x as i8).collect();
				Ok(NBT::new(&key, TAG::BYTE_ARRAY(buf)))
			},

			TAG_INT_ARRAY => {
				let count = self.read_signed_int()? as usize;
				let mut buf: Vec<i32> = Vec::with_capacity(count);
				for _ in 0..count {
					buf.push(self.read_signed_int()?);
				}
				Ok(NBT::new(&key, TAG::INT_ARRAY(buf)))
			},

			TAG_LONG_ARRAY => {
				let count = self.read_signed_int()? as usize;
				let mut buf: Vec<i64> = Vec::with_capacity(count);
				for _ in 0..count {
					buf.push(self.read_signed_long()?);
				}
				Ok(NBT::new(&key, TAG::LONG_ARRAY(buf)))
			},

			TAG_LIST => {
				byte = self.read_byte()?;
				let mut buf: Vec<NBT> = Vec::new();
				let count = self.read_signed_int()? as usize;
				for _ in 0..count {
					buf.push(self.read_nbt(false, inet, Some(byte))?);
				}
				Ok(NBT::new(&key, TAG::LIST(buf)))
			}


			_ => Err(DataError::NBTError)
		}

	}
}