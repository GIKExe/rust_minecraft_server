use std::{collections::HashMap, io::Cursor};

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

#[derive(Debug)]
pub enum TAG {
	END,
	COMPOUND(HashMap<String, TAG>),
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

#[derive(Debug)]
pub struct NBT {
	pub key: Option<String>,
	pub value: TAG,
}

impl NBT {
	fn new(key: Option<String>, value: TAG) -> Self {
		Self { key, value }
	}
}

pub trait NBT_Reader {
  fn _read_nbt(&mut self, root: bool, inet: bool, byte: Option<u8>) -> Result<NBT, DataError>;
	fn read_nbt(&mut self, inet: bool) -> Result<NBT, DataError>;
}


impl NBT_Reader for Cursor<Vec<u8>> {
	fn read_nbt(&mut self, inet: bool) -> Result<NBT, DataError> {
		self._read_nbt(true, inet, None)
	}

	fn _read_nbt(&mut self, root: bool, inet: bool, b: Option<u8>) -> Result<NBT, DataError> {
		let byte = match b { Some(v) => v, None => self.read_byte()? };
		let mut key: Option<String>;

		if byte == TAG_END {
			return Ok(NBT::new(None, TAG::END));
		}

		if root & (byte != TAG_COMPOUND) {
			return Err(DataError::NBTCompoundError)
		}

		if root & inet {
			key = None;
		} else {
			let size = self.read_short()? as usize;
			let buf = self.read_bytes(size)?;
			key = Some(String::from_utf8_lossy(&buf).to_string());
		}

		if b.is_some() {
			key = None;
		}

		match byte {
			TAG_BYTE => { Ok(NBT::new(key, TAG::BYTE(self.read_signed_byte()?))) },
			TAG_SHORT => { Ok(NBT::new(key, TAG::SHORT(self.read_signed_short()?))) },
			TAG_INT => { Ok(NBT::new(key, TAG::INT(self.read_signed_int()?))) },
			TAG_LONG => { Ok(NBT::new(key, TAG::LONG(self.read_signed_long()?))) },
			TAG_FLOAT => { Ok(NBT::new(key, TAG::FLOAT(self.read_float()?))) },
			TAG_DOUBLE => { Ok(NBT::new(key, TAG::DOUBLE(self.read_double()?))) },

			TAG_STRING => {
				let size = self.read_short()? as usize;
				let buf = self.read_bytes(size)?;
				Ok(NBT::new(key, TAG::STRING(String::from_utf8_lossy(&buf).to_string())))
			},

			TAG_BYTE_ARRAY => {
				let size = self.read_signed_int()? as usize;
				let buf =  self.read_bytes(size)?.into_iter().map(|x| x as i8).collect();
				Ok(NBT::new(key, TAG::BYTE_ARRAY(buf)))
			},

			TAG_INT_ARRAY => {
				let count = self.read_signed_int()? as usize;
				let mut buf: Vec<i32> = Vec::with_capacity(count);
				for _ in 0..count {
					buf.push(self.read_signed_int()?);
				}
				Ok(NBT::new(key, TAG::INT_ARRAY(buf)))
			},

			TAG_LONG_ARRAY => {
				let count = self.read_signed_int()? as usize;
				let mut buf: Vec<i64> = Vec::with_capacity(count);
				for _ in 0..count {
					buf.push(self.read_signed_long()?);
				}
				Ok(NBT::new(key, TAG::LONG_ARRAY(buf)))
			},

			TAG_LIST => {
				// byte = self.read_byte()?;
				let mut buf: Vec<NBT> = Vec::new();
				let count = self.read_signed_int()? as usize;
				println!("List count: {count}");
				for _ in 0..count {
					buf.push(self._read_nbt(false, inet, None)?);
				}
				Ok(NBT::new(key, TAG::LIST(buf)))
			},

			TAG_COMPOUND => {
				// println!("COMPOUND");
				let mut map = HashMap::new();
				loop {
					let nbt = self._read_nbt(false, inet, None)?;
					if matches!(nbt.value, TAG::END) { break; };
					let Some(key) = nbt.key else { continue; };
					map.insert(key, nbt.value);
				}
				Ok(NBT::new(key, TAG::COMPOUND(map)))
			}

			_ => {
				// println!("o: {other}");
				Err(DataError::NBTError)
			}
		}

	}
}