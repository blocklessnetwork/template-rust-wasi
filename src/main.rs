// Entry point to our WASI application

fn main() {
	let args = std::env::vars();
	for (key, val) in args {
		println!("{key}={val}");
	}

}
