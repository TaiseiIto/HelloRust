#[derive(PartialEq, Debug)]
pub struct Shoe {
	size: u32,
	style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, my_shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|shoe| shoe.size == my_shoe_size).collect()
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
}

