pub trait Integer {
	const BYTES: usize;
	fn from_bytes(array: &[u8]) -> Self;
}

impl Integer for i8 {
	const BYTES: usize = 1;
	fn from_bytes(array: &[u8]) -> Self {
		array[0] as i8
	}
}

impl Integer for i16 {
	const BYTES: usize = 2;
	fn from_bytes(array: &[u8]) -> Self {
		Self::from_be_bytes(array.try_into().unwrap())
	}
}

impl Integer for i32 {
	const BYTES: usize = 4;
	fn from_bytes(array: &[u8]) -> Self {
		Self::from_be_bytes(array.try_into().unwrap())
	}
}

impl Integer for i64 {
	const BYTES: usize = 8;
	fn from_bytes(array: &[u8]) -> Self {
		Self::from_be_bytes(array.try_into().unwrap())
	}
}
