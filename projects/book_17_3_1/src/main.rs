extern crate book_17_3_1;

use book_17_3_1::*;

fn main() {
	let mut post = Post::new();
	post.add_text("I ate a salad for lunch today.");
    let post = post.request_review();
    let post = post.reject();
    let post = post.request_review();
    let post = post.approve();
    let post = post.reject();
    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
	println!("post.content() = \"{}\"", post.content());
}

