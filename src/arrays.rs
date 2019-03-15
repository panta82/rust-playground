fn main() {
	let mut a = [10, 30, 20, 40];

	println!("Before: {:?}", a);

	for i in 0..a.len() - 1 {
		for j in i + 1..a.len() {
			if a[j] > a[i] {
				let tmp = a[j];
				a[j] = a[i];
				a[i] = tmp;
			}
		}
	}

	println!("After manual: {:?}", a);

	sort(&mut a);

	println!("After sliced: {:?}", a);
}

fn sort(arr: &mut [i32]) {
	for i in 0..arr.len() - 1 {
		for j in i + 1..arr.len() {
			if arr[j] < arr[i] {
				let tmp = arr[j];
				arr[j] = arr[i];
				arr[i] = tmp;
			}
		}
	}
}