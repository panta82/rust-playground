fn main() {
	let s1 = "String slice, static";
	dump(s1);

	let s2 = s1.to_string();
	dump(&s2);

	println!("{}", arr_to_str(&[]));
	println!("{}", arr_to_str(&[1, 2, 3]));
}

fn dump(s: &str) {
	println!("str '{}'", s);
}

fn arr_to_str(arr: &[i32]) -> String {
	let mut res = '['.to_string();
	for val in arr {
		res += &val.to_string();
		res.push(',');
	}
	if arr.len() > 0 {
		res.pop();
	}
	res.push(']');
	return res;
}