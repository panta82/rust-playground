fn main() {
	let mut v = Vec::new();
	v.push(5);
	v.push(6);
	v.push(7);

	for el in v.iter() {
		println!("{}", el);
	}

	for el in v.windows(2) {
		println!("win: {:?}", el);
	}

	for el in v.chunks(2) {
		println!("chunk: {:?}", el);
	}

	print_slice(&v);
}

fn print_slice(slice: &[i32]) {
	println!("slice is {:?}", slice);
}