fn largest(numbers: Vec<i32>) -> Result<i32, ()> {
	match numbers.get(0) {
		Some(number) => {
			let mut largest: i32 = *number;
			for number in numbers {
				if largest < number {
					largest = number;
				}
			}
			Ok(largest)
		},
		None => Err(()),
	}
}

fn main() {
	let numbers: Vec<i32> = vec![34, 50, 25, 100, 65];
	let largest_number: i32 = largest(numbers).expect("There is no largest number.");
	println!("The largest number is {}.", largest_number);
	let numbers: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let largest_number: i32 = largest(numbers).expect("There is no largest number.");
	println!("The largest number is {}.", largest_number);
}

