use std::io::prelude::*;

fn main() {
	let mut args: std::env::Args = std::env::args();
	let program_name: String = args.next().expect("There is no program name.");
	let query: String = args.next().expect("There is no query.");
	let file_name: String = args.next().expect("There is no file name.");
	println!("program_name = \"{}\"", program_name);
	println!("query = \"{}\"", query);
	println!("file_name = \"{}\"", file_name);
	let mut file: std::fs::File = std::fs::File::open(&file_name).expect(&format!("\"{}\" is not found.", &file_name));
	let mut contents: String = String::new();
	file.read_to_string(&mut contents).expect(&format!("Something went wrong reading \"{}\"", &file_name));
	println!("Contents of \"{}\"", &file_name);
	println!("{}", contents);
}

