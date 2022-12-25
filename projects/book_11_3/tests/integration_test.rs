extern crate book_11_3;
use book_11_3::*;
mod common;
use common::*;

#[test]
fn it_adds_two() {
	setup();
	assert_eq!(add_two(2), 4);
}

