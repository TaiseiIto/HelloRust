extern crate minigrep;

fn main() {
	let config: minigrep::Config = minigrep::Config::new(std::env::args()).unwrap_or_else(|error| {
		eprintln!("Error while parsing arguments: {}", error);
		std::process::exit(1);
	});
	if let Err(error) = minigrep::run(&config) {
		eprintln!("Error while running: {}", error);
		std::process::exit(2);
	}
}

