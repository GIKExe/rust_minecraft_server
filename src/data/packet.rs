use std::io::Cursor;

use super::{DataError, Reader, Writer};

#[derive(Debug, Clone)]
pub struct Packet {
	id: u8,
	cursor: Cursor<Vec<u8>>,
}

impl Packet {
	/// Create new packet from id and buffer
	pub fn new(id: u8, cursor: Cursor<Vec<u8>>) -> Packet {
		Packet { id, cursor }
	}

	/// Create new packet with id and empty buffer
	pub fn empty(id: u8) -> Packet {
		Packet {
			id,
			cursor: Cursor::new(Vec::new()),
		}
	}

	/// Build packet with lambda
	pub fn build<F>(id: u8, builder: F) -> Result<Packet, DataError>
	where
		F: FnOnce(&mut Packet) -> Result<(), DataError>,
	{
		let mut packet = Self::empty(id);
		builder(&mut packet)?;
		Ok(packet)
	}

	/// Get packet id
	pub fn id(&self) -> u8 {
		self.id
	}

	/// Set packet id
	pub fn set_id(&mut self, id: u8) {
		self.id = id;
	}

	/// Set packet cursor
	pub fn set_cursor(&mut self, cursor: Cursor<Vec<u8>>) {
		self.cursor = cursor;
	}

	/// Get cursor length
	pub fn len(&self) -> usize {
		self.get_bytes().len() - self.cursor.position() as usize
	}

	/// Is cursor empty
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Get cursor remaining bytes
	pub fn get_bytes(&self) -> &[u8] {
		self.cursor.get_ref()
	}

	/// Get mutable reference to cursor
	pub fn get_mut(&mut self) -> &mut Cursor<Vec<u8>> {
		&mut self.cursor
	}

	/// Get immutable reference to cursor
	pub fn get_ref(&self) -> &Cursor<Vec<u8>> {
		&self.cursor
	}

	/// Get inner cursor
	pub fn into_inner(self) -> Cursor<Vec<u8>> {
		self.cursor
	}
}

impl From<Packet> for Cursor<Vec<u8>> {
	fn from(val: Packet) -> Self {
		val.cursor
	}
}

impl Reader for Packet {
	fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, DataError> {
		self.cursor.read_bytes(size)
	}
}

impl Writer for Packet {
	fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), DataError> {
		self.cursor.write_bytes(bytes)
	}
}
