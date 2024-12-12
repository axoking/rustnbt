use std::{fs::File, io::BufReader, path::Path};

use crate::error::NBTError;

enum Tag {
	Compound(Vec<(String, Tag)>),
	List(Vec<Tag>),
	Boolean(bool),
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	String(String),
	ByteArray(Vec<i8>),
	IntArray(Vec<i32>),
	LongArray(Vec<i64>)
}

pub struct Decoder {
	reader: BufReader<File>,
	pos: usize
}

impl Decoder {
	pub fn open_file(path: impl AsRef<Path>) -> Result<Self, NBTError> {
		Ok(Self {
			reader: BufReader::new(File::open(path)?),
			pos: 0
		})
	}
}
