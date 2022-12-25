extern crate minigrep;

fn main() {
	let config: minigrep::Config = minigrep::Config::new(&mut std::env::args()).unwrap_or_else(|error| {
		println!("Error while parsing arguments: {}", error);
		std::process::exit(1);
	});
	if let Err(error) = minigrep::run(&config) {
		println!("Error while running: {}", error);
		std::process::exit(2);
	}
}

