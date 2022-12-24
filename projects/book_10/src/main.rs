fn main() {
	let numbers: Vec<i32> = vec![34, 50, 25, 100, 65];
	let mut largest: i32 = numbers[0];
	for number in numbers {
		if largest < number {
			largest = number;
		}
	}
	println!("The largest number is {}.", largest);
}

