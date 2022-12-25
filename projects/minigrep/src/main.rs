fn main() {
	let mut args: std::env::Args = std::env::args();
	let program_name: String = args.next().expect("There is no program name.");
	let query: String = args.next().expect("There is no query.");
	let file_name: String = args.next().expect("There is no file name.");
	println!("program_name = \"{}\"", program_name);
	println!("query = \"{}\"", query);
	println!("file_name = \"{}\"", file_name);
}

