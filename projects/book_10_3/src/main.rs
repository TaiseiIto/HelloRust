#[derive(Debug)]
struct ImportantExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
	fn get_part(&'a self) -> &'a str {
		self.part
	}
}

fn longest<'a>(str0: &'a str, str1: &'a str) -> &'a str {
	if str0.len() < str1.len() {
		str1
	} else {
		str0
	}
}

fn main() {
	println!("The longest string is \"{}\"", longest("abcd", "xyz"));
	let novel = "Call me Ishmael. Some years ago ...".to_string();
	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	let important_excerpt = ImportantExcerpt {
		part: first_sentence,
	};
	println!("Important excerpt = {:?}", important_excerpt);
	println!("important_excerpt.get_part() = {}", important_excerpt.get_part());
}

