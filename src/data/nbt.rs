use std::io::Cursor;




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

enum TAG {
	END,
	COMPOUND(Vec<NBT>),
	STRING(String),
	BYTE(i8),
	SHORT(i16),
	INT(i32),
	LONG(i64),
	FLOAT(f32),
	DOUBLE(f64),
	// LIST
	// BYTE_ARRAY
	// INT_ARRAY
	// LONG_ARRAY
}

struct NBT {
	key: String,
	value: TAG,
}

impl NBT {
	fn new(key: String, value: TAG) -> Self {
		Self { key, value }
	}
}

pub trait NBT_Reader {

}

impl NBT_Reader for Cursor<Vec<u8>> {

}