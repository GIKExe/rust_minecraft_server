use std::io::{Read, Write};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

use crate::inet::InetError;

mod async_reader;
mod reader;
mod async_writer;
mod writer;

mod packet;
mod packet_id;
mod component;
mod nbt;

#[derive(Debug)]
pub enum DataError {
	ReadError,
	WriteError,
	VarIntIsSoBig,
	StringDecodeError,
	Inet(InetError),
	SerializationError,
	DeSerializationError,
	ZlibError,
}

pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, DataError> {
	let mut decoder = ZlibDecoder::new(bytes);
	let mut output = Vec::new();
	decoder.read_to_end(&mut output).or(Err(DataError::ZlibError))?;
	Ok(output)
}

pub fn compress(bytes: &[u8]) -> Result<Vec<u8>, DataError> {
	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(1));
	encoder.write_all(bytes).or(Err(DataError::ZlibError))?;
	encoder.finish().or(Err(DataError::ZlibError))
}

pub use async_reader::*;
pub use reader::*;
pub use async_writer::*;
pub use writer::*;
pub use packet::*;
pub use packet_id::{clientbound, serverbound};
pub use component::*;
pub use nbt::*;