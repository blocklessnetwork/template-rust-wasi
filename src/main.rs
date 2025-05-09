// Entry point to our WASI application

use std::io::Write;

fn main() {
	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.create(true)
		.open("output.txt")
		.expect("Failed to open file");
	file.write_all(b"Hello World, WASI!")
		.expect("Failed to write to file");
}