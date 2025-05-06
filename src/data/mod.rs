use std::io::Read;

use craftflow_nbt::DynNBT;
use rust_mc_proto::{DataReader, DataWriter, Packet};

use super::ServerError;

pub mod text_component;

// Трейт для чтения NBT-совместимых приколов
pub trait ReadWriteNBT<T>: DataReader + DataWriter {
	fn read_nbt(&mut self) -> Result<T, ServerError>;
	fn write_nbt(&mut self, val: &T) -> Result<(), ServerError>;
}

impl ReadWriteNBT<DynNBT> for Packet {
	fn read_nbt(&mut self) -> Result<DynNBT, ServerError> {
		let mut data = Vec::new();
		let pos = self.get_ref().position();
		self
			.get_mut()
			.read_to_end(&mut data)
			.map_err(|_| ServerError::DeNbt)?;
		let (remaining, value) = craftflow_nbt::from_slice(&data).map_err(|_| ServerError::DeNbt)?;
		self
			.get_mut()
			.set_position(pos + (data.len() - remaining.len()) as u64);
		Ok(value)
	}

	fn write_nbt(&mut self, val: &DynNBT) -> Result<(), ServerError> {
		craftflow_nbt::to_writer(self.get_mut(), val).map_err(|_| ServerError::SerNbt)?;
		Ok(())
	}
}

pub trait ReadWritePosition: DataReader + DataWriter {
	fn read_position(&mut self) -> Result<(i64, i64, i64), ServerError>;
	fn write_position(&mut self, x: i64, y: i64, z: i64) -> Result<(), ServerError>;
}

impl ReadWritePosition for Packet {
	fn read_position(&mut self) -> Result<(i64, i64, i64), ServerError> {
		let val = self.read_long()?;
		Ok((val >> 38, val << 52 >> 52, val << 26 >> 38))
	}

	fn write_position(&mut self, x: i64, y: i64, z: i64) -> Result<(), ServerError> {
		Ok(self.write_long(((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF))?)
	}
}
