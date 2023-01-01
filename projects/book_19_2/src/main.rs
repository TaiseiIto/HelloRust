fn main() {
	let p = Point::new(1, 2);
	let q = Point::new(3, 4);
	let r = p + q;
	println!("r = {:?}", r);
}

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn new(x: i32, y: i32) -> Self {
		Self {x, y}
	}
}

impl std::ops::Add for Point {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

