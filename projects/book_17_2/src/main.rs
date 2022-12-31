extern crate book_17_2;
use book_17_2::*;

#[derive(Debug)]
struct SelectBox {
	width: u32,
	height: u32,
	options: Vec<String>,
}

impl Draw for SelectBox {
	fn draw(&self) {
		println!("Draw {:#?}", self);
	}
}

fn main() {
}

