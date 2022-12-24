use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error> {
	let file_name: &str = "hello.txt";
	let mut file: std::fs::File = match std::fs::File::open(file_name) {
		Ok(file) => file,
		Err(error) => return Err(error),
	};
	let mut username: String = String::new();
	match file.read_to_string(&mut username) {
		Ok(_) => Ok(username),
		Err(error) => Err(error),
	}
}

fn main() {
	let file_name: &str = "hello.txt";
	match std::fs::File::open(file_name) {
		Ok(file) => file,
		Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => match std::fs::File::create(file_name) {
			Ok(file) => file,
			Err(error) => panic!("Tried to create file but there was a problem: {:?}", error),
		}
		Err(error) => panic!("There was a problem opening the file: {:?}", error),
	};
	match read_username_from_file() {
		Ok(username) => println!("username: {}", username),
		Err(error) => panic!("Can't read user name: {:?}", error),
	}
}

