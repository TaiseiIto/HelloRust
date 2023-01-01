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
	println!("match_range(0) = {}", match_range(0));
	println!("match_range(1) = {}", match_range(1));
	println!("match_range(5) = {}", match_range(5));
	println!("match_range(6) = {}", match_range(6));
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

fn match_range(x: i32) -> &'static str {
	match x {
		1..=5 => "one through five",
		_ => "other",
	}
}

