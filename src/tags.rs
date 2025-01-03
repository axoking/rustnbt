use crate::indentation::IndentationType;

use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
pub enum Tag {
	Compound(HashMap<String, Tag>),
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

impl Tag {
	fn write_snbt_part(&self, target: &mut impl Write, indent: IndentationType, level: u16) -> Result<(), io::Error> {
		match self {
			Self::Boolean(val) => {
				write!(target, "{val}")?;
			}
			Self::Byte(val) => {
				write!(target, "{val}b")?;
			}
			Self::Short(val) => {
				write!(target, "{val}s")?;
			}
			Self::Int(val) => {
				write!(target, "{val}")?;
			}
			Self::Long(val) => {
				write!(target, "{val}l")?;
			}
			Self::Float(val) => {
				write!(target, "{val}f")?;
			}
			Self::Double(val) => {
				write!(target, "{val}d")?;
			}
			Self::String(val) => {
				write!(target, "\"{val}\"")?;
			}
			Self::ByteArray(array) => {
				write!(target, "[B; {}]", array.iter().map(|val| format!("{val}b")).collect::<Vec<_>>().join(", "))?;
			}
			Self::IntArray(array) => {
				write!(target, "[I; {}]", array.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", "))?;
			}
			Self::LongArray(array) => {
				write!(target, "[L; {}]", array.iter().map(|val| format!("{val}l")).collect::<Vec<_>>().join(", "))?;
			}
			Self::List(list) => {
				if list.len() == 0 {
					write!(target, "[]")?;
					return Ok(());
				}

				write!(target, "[")?;
				for (i, val) in list.iter().enumerate() {
					indent.begin_line(target, level + 1)?;
					val.write_snbt_part(target, indent, level + 1)?;
					if i != list.len() - 1 {
						write!(target, ", ")?;
					}
				}
				indent.begin_line(target, level)?;
				write!(target, "]")?;
			}
			Self::Compound(comp) => {
				if comp.len() == 0 {
					write!(target, "{{}}")?;
					return Ok(());
				}

				write!(target, "{{")?;
				for (i, (key, val)) in comp.iter().enumerate() {
					indent.begin_line(target, level + 1)?;
					write!(target, "{key}: ")?;
					val.write_snbt_part(target, indent, level + 1)?;
					if i != comp.len() - 1 {
						write!(target, ", ")?;
					}
				}
				indent.begin_line(target, level)?;
				write!(target, "}}")?;
			}
		}

		Ok(())
	}

	pub fn write_snbt(&self, target: &mut impl Write, indent: IndentationType) -> Result<(), io::Error> {
		self.write_snbt_part(target, indent, 0)
	}
}