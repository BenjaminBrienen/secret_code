#![feature(slice_group_by)]

use std::{
	error::Error,
	fs::File,
	io::Read,
};

type ProgramError = Box<dyn Error>;
type Memory = Vec<u8>;

fn main() -> Result<(), ProgramError>
{
	let file_bytes = read_file_bytes("input.txt")?;
	let secret = parse_puzzle_into_secret(file_bytes)?;
	println!("{secret}");
	Ok(())
}

fn parse_puzzle_into_secret(file_bytes: Memory) -> Result<String, ProgramError>
{
	let byte_is_ascii_0_or_1 = |file_byte: &u8, _: &u8| byte_to_bit(*file_byte).is_some();
	let parsed = file_bytes
		.group_by(byte_is_ascii_0_or_1)
		.map(|plaintext_byte| parse_plaintext_byte(plaintext_byte))
		.collect();
	let result = String::from_utf8(parsed)?;
	Ok(result)
}

fn read_file_bytes(file_name: &str) -> Result<Memory, ProgramError>
{
	let mut buffer = Vec::new();
	File::open(file_name)?.read_to_end(&mut buffer)?;
	Ok(buffer)
}

fn parse_plaintext_byte(plaintext_byte: &[u8]) -> u8
{
	plaintext_byte
		.iter()
		.filter(|byte| byte_to_bit(**byte).is_some())
		.rev()
		.enumerate()
		.fold(0, |accumulator, index_byte| {
			let shifted_bit = byte_to_bit(*index_byte.1).unwrap_or_default() << index_byte.0;
			accumulator + shifted_bit
		})
}

fn byte_to_bit(byte: u8) -> Option<u8>
{
	if byte == b'0'
	{
		Some(0)
	}
	else if byte == b'1'
	{
		Some(1)
	}
	else
	{
		None
	}
}
