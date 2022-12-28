use std::io::prelude::*;

pub struct Config {
	query: String,
	file_name: String,
	case_sensitive: bool,
}

impl Config {
	pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
		let mut args: std::iter::Rev<std::env::Args> = args.rev();
		let file_name: String = args.next().ok_or("There is no file name.")?;
		let query: String = args.next().ok_or("There is no query.")?;
		let mut case_sensitive: bool = std::env::var("CASE_INSENSITIVE").is_err();
		for arg in args {
			let arg: Vec<char> = arg.chars().collect::<Vec<char>>();
			let mut arg_iter: std::slice::Iter<char> = arg.iter();
			match arg_iter.next() {
				Some('-') => match arg_iter.next() {
					Some('i') => case_sensitive = false,
					Some(_) => (),
					None => (),
				},
				Some(_) => (),
				None => (),
			}
		}
		Ok(Config{
			query,
			file_name,
			case_sensitive,
		})
	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
	let file_name: &str = &config.file_name;
    let query: &str = &config.query;
	let mut file: std::fs::File = std::fs::File::open(&file_name)?;
	let mut contents: String = String::new();
	file.read_to_string(&mut contents)?;
    let contents: &str = &contents;
    let matched_lines: String = match config.case_sensitive {
		true => search,
		false => search_case_insensitive,
	}(query, contents).join("\n");
	println!("{}", matched_lines);
	Ok(())
}

#[cfg(test)]
mod test {
	#[test]
	fn case_sensitive() {
		let query: &str = "duct";
		let contents_lines: Vec<&str> = vec![
			"Rust:",
			"safe, fast, productive.",
			"Pick three.",
			"Duct tape.",
		];
		let contents: &str = &contents_lines.join("\n");
		assert_eq!(vec![contents_lines[1]], super::search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query: &str = "rUsT";
		let contents_lines: Vec<&str> = vec![
			"Rust:",
			"safe, fast, productive.",
			"Pick three.",
			"Trust me.",
		];
		let contents: &str = &contents_lines.join("\n");
		assert_eq!(vec![contents_lines[0], contents_lines[3]], super::search_case_insensitive(query, contents));
	}
}

