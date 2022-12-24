fn main() {
	let file_name: &str = "hello.txt";
	let file: std::fs::File = match std::fs::File::open(file_name) {
		Ok(file) => file,
		Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => match std::fs::File::create(file_name) {
			Ok(file) => file,
			Err(error) => panic!("Tried to create file but there was a problem: {:?}", error),
		}
		Err(error) => panic!("There was a problem opening the file: {:?}", error),
	};
}

