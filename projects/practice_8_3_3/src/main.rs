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

fn main() {
	let mut command: String = String::new();
	print!("manage employees > ");
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut command).expect("Failed to read line");
	let command: Option<Command> = analyse_command(&command[..]);
	println!("command = {:?}", command);
}

