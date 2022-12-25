use std::io::prelude::*;

pub struct Config {
	query: String,
	file_name: String,
}

impl Config {
	pub fn new(args: &mut std::env::Args) -> Result<Config, &'static str> {
		args.next().ok_or("There is no program name.")?;
		let query: String = args.next().ok_or("There is no query.")?;
		let file_name: String = args.next().ok_or("There is no file name.")?;
		Ok(Config{query, file_name})
	}
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
	println!("query = \"{}\"", config.query);
	println!("file_name = \"{}\"", config.file_name);
	let file_name: &str = &config.file_name;
	let mut file: std::fs::File = std::fs::File::open(&file_name)?;
	let mut contents: String = String::new();
	file.read_to_string(&mut contents)?;
	println!("Contents of \"{}\"", file_name);
	println!("{}", contents);
	Ok(())
}

