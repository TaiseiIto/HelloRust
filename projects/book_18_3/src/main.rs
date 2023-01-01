fn main() {
	println!("match_literal(0) = {}", match_literal(0));
	println!("match_literal(1) = {}", match_literal(1));
	println!("match_literal(2) = {}", match_literal(2));
	println!("match_literal(3) = {}", match_literal(3));
}

fn match_literal(x: u32) -> &'static str {
	match x {
		0 => "zero",
		1 => "one",
		2 => "two",
		_ => "many",
	}
}

