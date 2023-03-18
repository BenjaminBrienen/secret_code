#![feature(slice_group_by)]
fn main()
{
	let mut buffer = Vec::new();
	std::io::Read::read_to_end(&mut std::fs::File::open("input.txt").unwrap(), &mut buffer).unwrap();
	println!("{}", String::from_utf8(buffer.group_by(|byte, _| byte_to_bit(*byte).is_some()).map(|plaintext_byte| plaintext_byte.iter().filter(|byte| byte_to_bit(**byte).is_some()).rev().enumerate().fold(0, |acc, index_byte| acc + (byte_to_bit(*index_byte.1).unwrap_or(0) << index_byte.0))).collect()).unwrap());
}
fn byte_to_bit(byte: u8) -> Option<u8>
{
	if      byte == b'0' { Some(0) }
	else if byte == b'1' { Some(1) }
	else                 { None }
}
