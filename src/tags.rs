use std::{fs::File, io::{self, BufReader, Read}, path::Path};

use crate::{error::NBTError, integer::Integer};

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
	LongArray(Vec<i64>),
	Empty
}

pub struct Decoder {
	reader: BufReader<File>,
	pos: usize
}

impl Decoder {
	pub fn open_file(path: impl AsRef<Path>) -> Result<Self, io::Error> {
		Ok(Self {
			reader: BufReader::new(File::open(path)?),
			pos: 0
		})
	}

	fn read_integer<T: Integer>(&mut self) -> Result<T, io::Error> {
		let mut buffer = [0; 8];
		let slice = &mut buffer[..T::BYTES];
		self.reader.read_exact(slice)?;
		self.pos += T::BYTES;
		Ok(T::from_bytes(slice))
	}

	fn read_integer_array<T: Integer>(&mut self) -> Result<Vec<T>, io::Error> {
		let mut array: Vec<T> = Vec::new();
		let length: i32 = self.read_integer()?;
		for _ in 0..length {
			array.push(self.read_integer()?);
		}
		Ok(array)
	}

	fn error(&self, msg: String) -> NBTError {
		NBTError::Decoding(msg, self.pos)
	}

	pub fn decode(&mut self) -> Result<Tag, NBTError> {
		match self.read_integer::<i8>()? {
			0 => Ok(Tag::Empty),
			1 => self.decode_byte(),
			2 => self.decode_short(),
			3 => self.decode_int(),
			4 => self.decode_long(),
			5 => self.decode_float(),
			6 => self.decode_double(),
			7 => self.decode_byte_array(),
			8 => self.decode_string(),

			11 => self.decode_int_array(),
			12 => self.decode_long_array(),
			code => Err(self.error(format!("Unknown tag type: {code:#02X}")))
		}
	}

	fn decode_byte(&mut self) -> Result<Tag, NBTError> {
		Ok(Tag::Byte(self.read_integer()?))
	}
	fn decode_short(&mut self) -> Result<Tag, NBTError> {
		Ok(Tag::Short(self.read_integer()?))
	}
	fn decode_int(&mut self) -> Result<Tag, NBTError> {
		Ok(Tag::Int(self.read_integer()?))
	}
	fn decode_long(&mut self) -> Result<Tag, NBTError> {
		Ok(Tag::Long(self.read_integer()?))
	}

	fn decode_float(&mut self) -> Result<Tag, NBTError> {
		let mut bytes = [0; 4];
		self.reader.read_exact(&mut bytes)?;
		self.pos += 4;
		Ok(Tag::Float(f32::from_be_bytes(bytes)))
	}
	fn decode_double(&mut self) -> Result<Tag, NBTError> {
		let mut bytes = [0; 8];
		self.reader.read_exact(&mut bytes)?;
		self.pos += 8;
		Ok(Tag::Double(f64::from_be_bytes(bytes)))
	}

	fn decode_byte_array(&mut self) -> Result<Tag, NBTError> {
		Ok(Tag::ByteArray(self.read_integer_array()?))
	}
	fn decode_int_array(&mut self) -> Result<Tag, NBTError> {
		Ok(Tag::IntArray(self.read_integer_array()?))
	}
	fn decode_long_array(&mut self) -> Result<Tag, NBTError> {
		Ok(Tag::LongArray(self.read_integer_array()?))
	}

	fn decode_string(&mut self) -> Result<Tag, NBTError> {
		let length = self.read_integer::<i16>()? as u16;
		let mut str = String::new();
		let mut chunk = (&mut self.reader).take(length.into());
		chunk.read_to_string(&mut str)?;
		self.pos += length as usize;
		Ok(Tag::String(str))
	}
}
