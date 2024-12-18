use std::{fmt::Display, io};

#[derive(Debug)]
pub enum NBTError {
	Io(io::Error),
	Decoding(String, usize)
}

impl From<io::Error> for NBTError {
	fn from(err: io::Error) -> Self {
		Self::Io(err)
	}
}

impl Display for NBTError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Io(err) => f.write_fmt(format_args!("Input/Output error: {}", err.to_string())),
			Self::Decoding(msg, pos) => f.write_fmt(format_args!("Decoding failed (just before byte {pos}): {msg}"))
		}
	}
}
