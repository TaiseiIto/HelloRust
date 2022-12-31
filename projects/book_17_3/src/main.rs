extern crate book_17_3;
use book_17_3::*;

fn main() {
	let mut post = Post::new();
	post.add_text("I ate a salad for lunch today.");
	println!("post.content() = {}", post.content());
	post.request_review();
	println!("post.content() = {}", post.content());
	post.approve();
	println!("post.content() = {}", post.content());
}

