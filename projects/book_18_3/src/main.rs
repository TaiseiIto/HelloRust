fn main() {
	println!("match_literal(0) = {}", match_literal(0));
	println!("match_literal(1) = {}", match_literal(1));
	println!("match_literal(2) = {}", match_literal(2));
	println!("match_literal(3) = {}", match_literal(3));
	shadowing();
	println!("match_or(0) = {}", match_or(0));
	println!("match_or(1) = {}", match_or(1));
	println!("match_or(2) = {}", match_or(2));
	println!("match_or(3) = {}", match_or(3));
	println!("match_int_range(0) = {}", match_int_range(0));
	println!("match_int_range(1) = {}", match_int_range(1));
	println!("match_int_range(5) = {}", match_int_range(5));
	println!("match_int_range(6) = {}", match_int_range(6));
	println!("match_char_range('a') = {}", match_char_range('a'));
	println!("match_char_range('j') = {}", match_char_range('j'));
	println!("match_char_range('k') = {}", match_char_range('k'));
	println!("match_char_range('z') = {}", match_char_range('z'));
	println!("match_char_range('0') = {}", match_char_range('0'));
	let p = Point {
		x: 0,
		y: 7,
	};
	match_struct(p);
}

fn match_literal(x: u32) -> &'static str {
	match x {
		0 => "zero",
		1 => "one",
		2 => "two",
		_ => "many",
	}
}

fn shadowing() {
	let x = Some(5);
	let y = 10;
	match x {
		Some(y) => println!("y = {}", y),
		_ => (),
	}
	println!("y = {}", y);
}

fn match_or(x: u32) -> &'static str {
	match x {
		0 | 1 => "zero or one",
		2 => "two",
		_ => "many",
	}
}

fn match_int_range(x: i32) -> &'static str {
	match x {
		1..=5 => "one through five",
		_ => "other",
	}
}

fn match_char_range(c: char) -> &'static str {
	match c {
		'a'..='j' => "early ASCII letter",
		'k'..='z' => "late ASCII letter",
		_ => "something else",
	}
}

struct Point {
	x: i32,
	y: i32,
}

fn match_struct(Point {x: a, y: b}: Point) {
	println!("point ({}, {})", a, b);
}

