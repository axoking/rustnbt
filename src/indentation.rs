use std::io::{self, Write};

#[derive(Debug, Copy, Clone)]
pub enum IndentationType {
	Tabs,
	Spaces(u8),
	SingleLine
}

impl IndentationType {
	pub fn begin_line(&self, target: &mut impl Write, level: u16) -> Result<(), io::Error> {
		match self {
			Self::Tabs => write!(target, "\n{}", "\t".repeat(level as usize)).map(|_| ()),
			Self::Spaces(spaces) => write!(target, "\n{}", " ".repeat(level as usize * *spaces as usize)).map(|_| ()),
			Self::SingleLine => Ok(())
		}
	}
}