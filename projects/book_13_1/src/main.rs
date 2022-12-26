fn generate_workout(intensity: u32, random_number: u32) {
	let expensive_closure: fn(u32) -> u32 = |num: u32| -> u32 {
		println!("Calculating slowly ...");
		std::thread::sleep(std::time::Duration::from_secs(2));
		num
	};
	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_closure(intensity));
		println!("Next, do {} situps!", expensive_closure(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_closure(intensity));
		}
	}
}

fn main() {
	let user_specified_value: u32 = 10;
	let random_number: u32 = 7;
	generate_workout(user_specified_value, random_number);
}

