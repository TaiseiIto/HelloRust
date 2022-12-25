#[derive(Debug)]
pub struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	pub fn can_hold(&self, other: &Rectangle) -> bool {
		other.width < self.width && other.height < self.height
	}

	pub fn get_width(&self) -> &u32 {
		&self.width
	}

	pub fn get_height(&self) -> &u32 {
		&self.height
	}
}

pub fn add_two(n: i32) -> i32 {
	n + 2
}

pub fn greet(name: &str) -> String {
	format!("Hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

	const LARGER: Rectangle = Rectangle {
		width: 8,
		height: 7,
	};
	const SMALLER: Rectangle = Rectangle {
		width: 5,
		height: 1,
	};

    #[test]
    fn larger_can_hold_smaller() {
		assert!(LARGER.can_hold(&SMALLER));
    }

	#[test]
	fn smaller_cannot_hold_larger() {
		assert!(!SMALLER.can_hold(&LARGER));
	}

	#[test]
	fn test_add_two() {
		assert_eq!(add_two(2), 4);
	}

	#[test]
	fn greeting_contains_name() {
		let name = "Taisei Ito";
		let greeting = greet(name);
		assert!(
			greeting.contains(name),
			"Greeting \"{}\" did not contains name \"{}\"",
			greeting,
			name
		);
	}
}

