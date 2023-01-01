fn main() {
	let mut stack: Vec<i32> = vec![1, 2, 3];
	while let Some(x) = stack.pop() {
		println!("x = {}", x);
	}
}

