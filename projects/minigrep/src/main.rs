use std::io::prelude::*;

struct Config {
	query: String,
	file_name: String,
}

impl Config {
	fn new(args: &mut std::env::Args) -> Result<Config, &'static str> {
		args.next().ok_or("There is no program name.")?;
		let query: String = args.next().ok_or("There is no query.")?;
		let file_name: String = args.next().ok_or("There is no file name.")?;
		Ok(Config{query, file_name})
	}
}

fn main() {
	let config: Config = Config::new(&mut std::env::args()).unwrap_or_else(|error| {
		println!("Error while parsing arguments: {}", error);
		std::process::exit(1);
	});
	println!("query = \"{}\"", config.query);
	println!("file_name = \"{}\"", config.file_name);
	let file_name: &str = &config.file_name;
	let mut file: std::fs::File = std::fs::File::open(&file_name).expect(&format!("\"{}\" is not found.", file_name));
	let mut contents: String = String::new();
	file.read_to_string(&mut contents).expect(&format!("Something went wrong reading \"{}\"", file_name));
	println!("Contents of \"{}\"", file_name);
	println!("{}", contents);
}

