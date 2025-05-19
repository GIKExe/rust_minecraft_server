pub mod reader;
pub mod writer;
mod tags;

use std::collections::HashMap;

pub use reader::*;
pub use writer::*;

#[derive(Debug)]
pub enum Tag {
	End,
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	ByteArray(Vec<i8>),
	String(String),
	List(Vec<Tag>),
	Compound(HashMap<String, Tag>),
	IntArray(Vec<i32>),
	LongArray(Vec<i64>),
}