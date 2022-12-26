struct Cacher<T> where T: Fn(u32) -> u32 {
	calculate: T,
	value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
	fn new(calculate: T) -> Cacher<T> {
		Cacher {
			calculate,
			value: None,
		}
	}

	fn value(&mut self, arg: u32) -> u32 {
		match self.value {
			Some(v) => v,
			None => {
				self.value = Some((self.calculate)(arg));
				self.value(arg)
			}
		}
	}
}

fn generate_workout(intensity: u32, random_number: u32) {
	let mut expensive_calculator: Cacher<fn(u32) -> u32> = Cacher::new(|num: u32| -> u32 {
		println!("Calculating slowly ...");
		std::thread::sleep(std::time::Duration::from_secs(2));
		num
	});
	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_calculator.value(intensity));
		println!("Next, do {} situps!", expensive_calculator.value(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_calculator.value(intensity));
		}
	}
}

fn main() {
	let user_specified_value: u32 = 10;
	let random_number: u32 = 7;
	generate_workout(user_specified_value, random_number);
}

