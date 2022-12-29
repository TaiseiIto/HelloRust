/// It adds one to the number given.
/// 
/// # Examples
///
/// '''
/// assert_eq(add_one(0), 1);
/// '''
pub fn add_one(x: i32) -> i32 {
	x + 1
}

#[cfg(test)]
mod test {
	#[test]
	fn add_one() {
		assert_eq!(super::add_one(0), 1);
	}
}

