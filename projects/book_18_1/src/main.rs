fn main() {
	let mut stack: Vec<i32> = vec![1, 2, 3];
	while let Some(x) = stack.pop() {
		println!("x = {}", x);
	}
	let vector: Vec<char> = vec!['a', 'b', 'c'];
	for (index, chr) in vector.iter().enumerate() {
		println!("vector[{}] = {}", index, chr);
	}
	let (x, y, ..) = (1, 2, 3, 4, 5);
	println!("x = {}", x);
	println!("y = {}", y);
	let point: (i32, i32) = (3, 5);
	print_coordinates(point);
}

fn print_coordinates((x, y): (i32, i32)) {
	println!("Current location: ({}, {})", x, y);
}

