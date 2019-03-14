fn main() {
	for i in 1..100 {
		let by_3 = i % 3 == 0;
		let by_5 = i % 5 == 0;
		if by_3 {
			print!("Fizz");
		}
		if by_5 {
			print!("Buzz");
		}
		if !by_3 && !by_5 {
			print!("{}", i);
		}

		println!();
	}
}