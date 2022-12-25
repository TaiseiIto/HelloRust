use std::io::prelude::*;

struct Config {
	query: String,
	file_name: String,
}

impl Config {
	fn new(args: &mut std::env::Args) -> Config {
		args.next().expect("There is no program name.");
		let query: String = args.next().expect("There is no query.");
		let file_name: String = args.next().expect("There is no file name.");
		Config {
			query,
			file_name
		}
	}
}

fn main() {
	let config: Config = Config::new(&mut std::env::args());
	println!("query = \"{}\"", config.query);
	println!("file_name = \"{}\"", config.file_name);
	let file_name: &str = &config.file_name;
	let mut file: std::fs::File = std::fs::File::open(&file_name).expect(&format!("\"{}\" is not found.", file_name));
	let mut contents: String = String::new();
	file.read_to_string(&mut contents).expect(&format!("Something went wrong reading \"{}\"", file_name));
	println!("Contents of \"{}\"", file_name);
	println!("{}", contents);
}

