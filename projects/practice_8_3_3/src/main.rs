use std::io::Write;

#[derive(Debug)]
enum Command {
	Quit,
}

fn analyse_command(line: &str) -> Option<Command> {
	let mut words: std::str::SplitWhitespace = line.split_whitespace();
	match words.next() {
		Some(command) => match &*command.to_lowercase() {
			"quit" => Some(Command::Quit),
			_ => None,
		},
		None => None,
	}
}

fn execute_command(command: Command) {
	match command {
		Command::Quit => println!("quit"),
	}
}

fn main() {
	let mut command: String = String::new();
	print!("manage employees > ");
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut command).expect("Failed to read line");
	match analyse_command(&*command) {
		Some(command) => execute_command(command),
		None => main(),
	}
}

