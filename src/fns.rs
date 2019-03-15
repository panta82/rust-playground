use std::f64::consts;

fn main() {
	let mut x = 1;
	println!("Before: {}", x);
	inc_by_ref(&mut x);
	inc_by_ref(&mut x);
	println!("After {}", x);

	let x = 2.0 * consts::PI;
	let abs_diff = (x.cos() - 1.0).abs();
	println!("Diff {}", abs_diff);
}

fn inc_by_ref(x: &mut i32) {
	*x += 1;
}