//! # book_14_2
//!
//! 'book_14_2' is a practice of [this section](https://doc.rust-jp.rs/book-ja/ch14-02-publishing-to-crates-io.html).

/// It adds one to the number given.
/// 
/// # Examples
///
/// ```
/// assert_eq!(book_14_2::add_one(0), 1);
/// ```
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

