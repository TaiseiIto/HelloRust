fn internal_adder(a: i32, b: i32) -> i32 {
	a + b
}

pub fn add_two(a: i32) -> i32 {
	internal_adder(a, 2)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_internal_adder() {
		assert_eq!(internal_adder(2, 2), 4);
	}

	#[test]
	fn test_add_two() {
		assert_eq!(add_two(2), 4);
	}
}

