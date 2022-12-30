//! # book_14_2
//!
//! 'book_14_2' is a practice of [this section](https://doc.rust-jp.rs/book-ja/ch14-02-publishing-to-crates-io.html).

/// It adds one to the number given.
/// 
/// # Examples
///
/// ```
/// assert_eq!(taisei_ito_book_14_2::add_one(0), 1);
/// ```
pub fn add_one(x: i32) -> i32 {
	x + 1
}

/// # Art
///
/// A library for modeling artistic concepts.
pub mod kinds {
	/// The primary colors according to the RYB color model.
	pub enum PrimaryColor {
		Red,
		Yellow,
		Blue,
	}

	/// The secondary colors according to the RYB color model.
	pub enum SecondaryColor {
		Primary {
			primary: PrimaryColor,
		},
		Orange,
		Green,
		Purple,
	}
}

pub mod utils {
	use super::kinds::*;

	/// It combines two primary colors in equal amounts to create a secondary color.
	pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
		match c1 {
			PrimaryColor::Red => match c2 {
				PrimaryColor::Red => SecondaryColor::Primary {
					primary: PrimaryColor::Red
				},
				PrimaryColor::Yellow => SecondaryColor::Orange,
				PrimaryColor::Blue => SecondaryColor::Purple,
			},
			PrimaryColor::Yellow => match c2 {
				PrimaryColor::Red => SecondaryColor::Orange,
				PrimaryColor::Yellow => SecondaryColor::Primary {
					primary: PrimaryColor::Yellow
				},
				PrimaryColor::Blue => SecondaryColor::Green,
			},
			PrimaryColor::Blue => match c2 {
				PrimaryColor::Red => SecondaryColor::Purple,
				PrimaryColor::Yellow => SecondaryColor::Green,
				PrimaryColor::Blue => SecondaryColor::Primary {
					primary: PrimaryColor::Blue
				},
			},
		}
	}
}

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

#[cfg(test)]
mod test {
	#[test]
	fn add_one() {
		assert_eq!(super::add_one(0), 1);
	}
}

