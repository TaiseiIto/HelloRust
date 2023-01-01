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
	let q = Point {
		x: 8,
		y: 0,
	};
	let r = Point {
		x: 9,
		y: 7,
	};
	match_struct(&p);
	println!("match_part_of_struct(&p) = {}", match_part_of_struct(&p));
	println!("match_part_of_struct(&q) = {}", match_part_of_struct(&q));
	println!("match_part_of_struct(&r) = {}", match_part_of_struct(&r));
	let ((feet, inches), Point{x, y}) = ((3, 10), Point{x: 3, y: -10});
	println!("feet = {}", feet);
	println!("inches = {}", inches);
	println!("x = {}", x);
	println!("y = {}", y);
	match_first_and_last(vec![]);
	match_first_and_last(vec![1]);
	match_first_and_last(vec![1,2,3,4,5]);
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

fn match_struct(&Point {x: a, y}: &Point) {
	println!("point ({}, {})", a, y);
}

fn match_part_of_struct(p: &Point) -> &'static str {
	match p {
		Point {
			x: 0,
			y: _,
		} => "On the y axix",
		Point {
			x: _,
			y: 0,
		} => "On the x axix",
		_ => "On neither axis"
	}
}

fn match_first_and_last(list: Vec<i32>) {
	match list[..] {
		[] => println!("nothing"),
		[x] => println!("only {}", x),
		[first, .., last] => {
			println!("first = {}", first);
			println!("last = {}", last);
		},
	};
}

