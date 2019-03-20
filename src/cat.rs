use std::{env, io};
use std::fs::File;
use std::io::Read;

fn main() {
	let filename = env::args()
			.nth(1)
			.expect("You must provide filename as first argument");

	let text = read_to_string_v2(&filename).expect("Failed to read file");

	println!("{0}", text);
}

fn read_to_string_v1(filename: &str) -> Result<String, io::Error> {
	let mut file = match File::open(&filename) {
		Ok(f) => f,
		Err(e) => return Err(e)
	};


	let mut text = String::new();
	match file.read_to_string(&mut text) {
		Ok(_) => Ok(text),
		Err(e) => Err(e)
	}
}

fn read_to_string_v2(filename: &str) -> io::Result<String> {
	let mut file = File::open(&filename)?;
	let mut text = String::new();
	file.read_to_string(&mut text)?;
	Ok(text)
}