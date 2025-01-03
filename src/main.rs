use nbt::decode::Decoder;
use nbt::indentation::IndentationType;

fn main() {
	let mut decoder = Decoder::open_file("level.dat").unwrap();
	let nbt = decoder.decode().unwrap();
	nbt.write_snbt(&mut std::io::stdout(), IndentationType::Spaces(4)).unwrap();
}
