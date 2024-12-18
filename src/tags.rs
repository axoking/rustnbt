#[derive(Debug)]
pub enum Tag {
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
