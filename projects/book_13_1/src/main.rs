struct Cacher<T> where T: Fn(u32) -> u32 {
	calculate: T,
	cache: std::collections::HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
	fn new(calculate: T) -> Cacher<T> {
		Cacher {
			calculate,
			cache: std::collections::HashMap::new(),
		}
	}

	fn get(&mut self, arg: u32) -> u32 {
		match self.cache.get(&arg) {
			Some(result) => *result,
			None => {
				self.cache.insert(arg, (self.calculate)(arg));
				self.get(arg)
			}
		}
	}
}

fn generate_workout(intensity: u32, random_number: u32) {
	let mut expensive_cache: Cacher<fn(u32) -> u32> = Cacher::new(|num: u32| -> u32 {
		println!("Calculating slowly ...");
		std::thread::sleep(std::time::Duration::from_secs(2));
		num
	});
	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_cache.get(intensity));
		println!("Next, do {} situps!", expensive_cache.get(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_cache.get(intensity));
		}
	}
}

fn main() {
	let user_specified_value: u32 = 10;
	let random_number: u32 = 7;
	generate_workout(user_specified_value, random_number);
}

