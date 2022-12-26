fn simulate_expensive_calculation(intensity: u32) -> u32 {
	println!("Calculating slowly ...");
	std::thread::sleep(std::time::Duration::from_secs(2));
	intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
	if intensity < 25 {
		println!("Today, do {} pushups!", simulate_expensive_calculation(intensity));
		println!("Next, do {} situps!", simulate_expensive_calculation(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", simulate_expensive_calculation(intensity));
		}
	}
}

fn main() {
	let user_specified_value: u32 = 10;
	let random_number: u32 = 7;
	generate_workout(user_specified_value, random_number);
}

