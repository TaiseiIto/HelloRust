fn main() {
	let p = Point::new(1, 2);
	let q = Point::new(3, 4);
	let r = p + q;
	println!("r = {:?}", r);
	r.outline_print();
	println!("Meters(1.0) + Meters(1.0) = {:?}", Meters(1.0) + Meters(1.0));
	println!("Meters(1.0) + Millimeters(1.0) = {:?}", Meters(1.0) + Millimeters(1.0));
	println!("Millimeters(1.0) + Meters(1.0) = {:?}", Millimeters(1.0) + Meters(1.0));
	println!("Millimeters(1.0) + Millimeters(1.0) = {:?}", Millimeters(1.0) + Millimeters(1.0));
	Pilot::fly(&Human);
	Wizard::fly(&Human);
	Human.fly();
	println!("A baby dog is called {}.", Dog::baby_name());
	println!("A baby dog is called {}.", <Dog as Animal>::baby_name());
	let w = Wrapper(["hello", "world"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());
	println!("w = {}", w);
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

impl std::fmt::Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl OutlinePrint for Point {}

#[derive(Debug)]
struct Meters(f64);

impl std::ops::Add for Meters {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self(self.0 + other.0)
	}
}

impl std::ops::Add<Millimeters> for Meters {
	type Output = Self;

	fn add(self, other: Millimeters) -> Self {
		Self(self.0 + other.0 / 1000 as f64)
	}
}

#[derive(Debug)]
struct Millimeters(f64);

impl std::ops::Add for Millimeters {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self(self.0 + other.0)
	}
}

impl std::ops::Add<Meters> for Millimeters {
	type Output = Self;

	fn add(self, other: Meters) -> Self {
		Self(self.0 + other.0 * 1000 as f64)
	}
}

trait Pilot {
	fn fly(&self);
}

trait Wizard {
	fn fly(&self);
}

struct Human;

impl Human {
	fn fly(&self) {
		println!("Wave arm furiously.");
	}
}

impl Pilot for Human {
	fn fly(&self) {
		println!("This is your captain speaking.");
	}
}

impl Wizard for Human {
	fn fly(&self) {
		println!("Up!");
	}
}

trait Animal {
	fn baby_name() -> String;
}

struct Dog;

impl Dog {
	fn baby_name() -> String {
		"Spot".to_string()
	}
}

impl Animal for Dog {
	fn baby_name() -> String {
		"puppy".to_string()
	}
}

trait OutlinePrint: std::fmt::Display {
	fn outline_print(&self) {
		let output = self.to_string();
		let len = output.len();
		println!("{}", "*".repeat(len + 4));
		println!("*{}*", " ".repeat(len + 2));
		println!("* {} *", output);
		println!("*{}*", " ".repeat(len + 2));
		println!("{}", "*".repeat(len + 4));
	}
}

struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "[{}]", self.0.join(", "))
	}
}

