fn main() {
	let some_var = 42;
	println!("Var is {}", some_var);

	assert_eq!(some_var, 42);

	println!("Sum of ints is {}", sum_ints(5));
	println!("Sum of floats is {}", sum_floats(5));
}

fn sum_ints(max: i32) -> i32 {
	let mut sum = 0;
	for i in 0..max {
		sum += i;
	}
	return sum;
}

fn sum_floats(max: i32) -> f64 {
	let mut sum = 0.0;
	for i in 0..max {
		sum += i as f64;
	}
	return sum;
}