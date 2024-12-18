use nbt::decode::Decoder;

fn main() {
	let mut decoder = Decoder::open_file("level.dat").unwrap();
	let nbt = decoder.decode().unwrap();
	println!("{nbt:?}");
}
