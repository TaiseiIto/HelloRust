struct Cacher<T, U> where T: Eq + std::hash::Hash {
	calculate: fn(T) -> U,
	cache: std::collections::HashMap<T, U>,
}

impl<T, U> Cacher<T, U> where T: Eq + std::hash::Hash {
	fn new(calculate: fn(T) -> U) -> Cacher<T, U> {
		Cacher {
			calculate,
			cache: std::collections::HashMap::new(),
		}
	}

	fn get(self: &mut Self, arg: &T) -> U where T: Copy, U: Copy {
		match self.cache.get(arg) {
			Some(result) => *result,
			None => {
				self.cache.insert(*arg, (self.calculate)(*arg));
				self.get(arg)
			}
		}
	}
}

fn generate_workout(intensity: u32, random_number: u32) {
	let mut expensive_cache: Cacher<u32, u32> = Cacher::new(|num: u32| -> u32 {
		println!("Calculating slowly ...");
		std::thread::sleep(std::time::Duration::from_secs(2));
		num
	});
	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_cache.get(&intensity));
		println!("Next, do {} situps!", expensive_cache.get(&intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_cache.get(&intensity));
		}
	}
}

fn main() {
	let user_specified_value: u32 = 10;
	let random_number: u32 = 7;
	generate_workout(user_specified_value, random_number);
}

