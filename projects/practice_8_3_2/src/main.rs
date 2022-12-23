use std::io::Write;

#[derive(Debug)]
enum Phoneme {
	Vowel,
	Consonant,
}

fn char2phoneme(c: char) -> Option<Phoneme> {
	match c.to_lowercase().collect::<Vec<char>>().get(0) {
		Some(l) => match l {
			'a' |
			'e' |
			'i' |
			'o' |
			'u' => Some(Phoneme::Vowel),
			'b' |
			'c' |
			'd' |
			'f' |
			'g' |
			'h' |
			'j' |
			'k' |
			'l' |
			'm' |
			'n' |
			'p' |
			'q' |
			'r' |
			's' |
			't' |
			'v' |
			'w' |
			'x' |
			'y' |
			'z' => Some(Phoneme::Consonant),
			_ => None,
		},
		None => None,
	}
}

fn main() {
	let mut text: String = String::new();
	print!("Input some test : ");
    std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut text).expect("Failed to read line");
	let text: String = (&text[0..text.len() - 1]).to_string();
	println!("text = \"{}\"", text);
	let mut phonemes: Vec<Phoneme> = Vec::new();
	for c in text.chars() {
		if let Some(p) = char2phoneme(c) {
			phonemes.push(p);
		}
	}
	println!("phonemes = {:?}", phonemes);
}

