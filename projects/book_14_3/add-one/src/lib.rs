pub fn add_one(x: i32) -> i32 {
	x + 1
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(super::add_one(2), 3);
	}
}

