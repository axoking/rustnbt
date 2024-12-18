use std::{io::{self, BufReader, Read}, fs::File, path::Path};
use crate::{integer::Integer, error::NBTError, tags::Tag};

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

	fn decode_by_id(&mut self, id: i8) -> Result<Tag, NBTError> {
		match id {
			1 => self.decode_byte(),
			2 => self.decode_short(),
			3 => self.decode_int(),
			4 => self.decode_long(),
			5 => self.decode_float(),
			6 => self.decode_double(),
			7 => self.decode_byte_array(),
			8 => self.decode_string(),
			9 => self.decode_list(),
			10 => self.decode_compound(),
			11 => self.decode_int_array(),
			12 => self.decode_long_array(),
			code => Err(self.error(format!("Invalid tag type: {code:#02X}")))
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

	fn decode_list(&mut self) -> Result<Tag, NBTError> {
		let tag_type: i8 = self.read_integer()?;
		let length: i32 = self.read_integer()?;
		let mut list = Vec::new();
		for _ in 0..length {
			list.push(self.decode_by_id(tag_type)?);
		}
		Ok(Tag::List(list))
	}

	fn decode_compound(&mut self) -> Result<Tag, NBTError> {
		let mut data = Vec::new();
		loop {
			match self.decode_compound_element()? {
				Some(element) => data.push(element),
				None => return Ok(Tag::Compound(data))
			}
		}
	}

	fn decode_compound_element(&mut self) -> Result<Option<(String, Tag)>, NBTError> {
		let tag_type: i8 = self.read_integer()?;
		if tag_type == 0 {
			return Ok(None);
		}

		let length = self.read_integer::<i16>()? as u16;
		let mut name = String::new();
		let mut chunk = (&mut self.reader).take(length.into());
		chunk.read_to_string(&mut name)?;
		self.pos += length as usize;

		let content = self.decode_by_id(tag_type)?;
		Ok(Some((name, content)))
	}

	pub fn decode(&mut self) -> Result<Tag, NBTError> {
		match self.decode_compound_element()? {
			Some((_, tag)) => Ok(tag),
			None => Err(self.error("File started with an end tag".to_owned()))
		}
	}
}
