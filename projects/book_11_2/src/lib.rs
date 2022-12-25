pub fn print_and_return_10(a: i32) -> i32 {
	println!("I got the value {}.", a);
	10
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn this_test_will_pass() {
		assert_eq!(print_and_return_10(1), 10);
	}

	#[test]
	fn this_test_will_fail() {
		assert_eq!(print_and_return_10(0), 0);
	}
}

