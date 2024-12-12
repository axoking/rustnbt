use std::{fs::File, io::{self, BufReader, Read}, path::Path};

enum TokenContent {
	BooleanValue(bool),
	NumeralValue(String),
	DecimalPoint,

	ByteIndicator,
	IntIndicator,
	ShortIndicator,
	LongIndicator,
	FloatIndicator,
	DoubleIndicator,

	QuotedText(String),
	UnquotedText(String),

	ListOpener,
	ListCloser,
	CompoundOpener,
	CompoundCloser,

	Seperator,
	ArrayTypeSuffix,
	TagNameSuffix
}

struct Token {
	content: TokenContent,
	line: u32,
	column: u32
}

struct Lexer {
	reader: BufReader<File>,
	line: u32,
	column: u32
}

impl Lexer {
	pub fn open(path: impl AsRef<Path>) -> Result<Self, io::Error> {
		Ok(Lexer {
			reader: BufReader::new(std::fs::File::open(path)?),
			line: 0,
			column: 0
		})
	}

	fn get_token(&mut self) -> Result<Token, io::Error> {
		let mut byte = [0];
		self.reader.read_exact(&mut byte)?;

		match byte[0] {
			// bro why tf am I writing an snbt compiler here I wanted to write a raw nbt interpreter first
		}
	}
}
