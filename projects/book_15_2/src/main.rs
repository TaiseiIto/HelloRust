struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> std::ops::Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

fn main() {
	let x = 5;
	let y = MyBox::new(x);
	println!("x = {}", x);
	println!("*y = {}", *y);
}

