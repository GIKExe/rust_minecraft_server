use std::io::Read;

use crate::inet::InetError;

mod async_reader;
mod reader;
mod writer;
mod packet;
mod packet_id;
mod component;

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

pub use async_reader::*;
pub use reader::*;
use flate2::bufread::ZlibDecoder;
pub use writer::*;
pub use packet::*;
pub use packet_id::{clientbound, serverbound};
pub use component::*;