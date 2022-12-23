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

fn pig_latin_word(word: &str) -> String {
	match word.chars().nth(0) {
		Some(c) => match char2phoneme(c) {
			Some(Phoneme::Vowel) => format!("{}{}", word.to_string(), "hay".to_string()),
			Some(Phoneme::Consonant) => format!("{}{}{}", (&word[1..]).to_string(), c.to_string(), "ay".to_string()),
			None => String::new(),
		},
		None => String::new(),
	}
}

fn pig_latin(text: &str) -> String {
	text
		.split_whitespace().map(|word| pig_latin_word(&word[..])).fold(String::new(), |sentence, word| format!("{} {}", sentence, word))
}

fn main() {
	let mut text: String = String::new();
	print!("Input some test : ");
    std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut text).expect("Failed to read line");
	let text: String = (&text[..text.len() - 1]).to_string();
	println!("text = \"{}\"", text);
	println!("pig_latin(&text[..]) = \"{}\"", pig_latin(&text[..]));
}

