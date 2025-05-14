// Entry point to our WASI application

fn main() {
	let content = std::fs::read_to_string("./input.txt")
		.expect("Failed to read file");
	println!("File content: {content}");
}