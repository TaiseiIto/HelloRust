#[derive(PartialEq, Debug)]
pub struct Shoe {
	size: u32,
	style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, my_shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|shoe| shoe.size == my_shoe_size).collect()
}

pub struct Counter {
	count: u32,
}

impl Counter {
	pub fn new() -> Counter {
		Counter {
			count: 0,
		}
	}
}

impl Iterator for Counter {
	type Item = u32;

	fn next(self: &mut Self) -> Option<Self::Item> {
		self.count += 1;
		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn iterator_demonstration() {
		let v = vec![1, 2, 3];
		let mut iter = v.iter();
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn iterator_sum() {
		assert_eq!(vec![1, 2, 3].iter().sum::<i32>(), 6);
	}

	#[test]
	fn iterator_map() {
		assert_eq!(vec![1, 2, 3].iter().map(|x| x + 1).collect::<Vec<i32>>(), vec![2, 3, 4]);
	}

	#[test]
	fn filter_by_size() {
		let shoes: Vec<super::Shoe> = vec![
			super::Shoe {
				size: 10,
				style: "sneaker".to_string(),
			},
			super::Shoe {
				size: 13,
				style: "sandal".to_string(),
			},
			super::Shoe {
				size: 10,
				style: "boot".to_string(),
			},
		];
		let in_my_size = super::shoes_in_my_size(shoes, 10);
		assert_eq!(in_my_size, vec![
			super::Shoe {
				size: 10,
				style: "sneaker".to_string(),
			},
			super::Shoe {
				size: 10,
				style: "boot".to_string(),
			},
		]);
	}

	#[test]
	fn call_next_directly() {
		let mut counter: super::Counter = super::Counter::new();
		assert_eq!(counter.next(), Some(1));
		assert_eq!(counter.next(), Some(2));
		assert_eq!(counter.next(), Some(3));
		assert_eq!(counter.next(), Some(4));
		assert_eq!(counter.next(), Some(5));
		assert_eq!(counter.next(), None);
	}

	#[test]
	fn use_other_iterator_trait_methods() {
		assert_eq!(super::Counter::new().zip(super::Counter::new().skip(1)).map(|(a, b)| a * b).filter(|x| x % 3 == 0).sum::<u32>(), 18);
	}
}

