// Entry point to our WASI application


fn main() {
	let vec = Vec::<u8>::with_capacity(1024*1024*5);
	println!("Allocated 5MB of memory: {}", vec.capacity());
}