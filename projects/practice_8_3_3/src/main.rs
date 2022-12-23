use std::io::Write;

#[derive(Debug)]
enum Command {
	Add {employee: String, department: String},
	List {department: Option<String>},
	Quit,
}

fn analyse_command(line: &str) -> Option<Command> {
	let mut words: std::str::SplitWhitespace = line.split_whitespace();
	match words.next() {
		Some(command) => match &*command.to_lowercase() {
			"add" => match words.next() {
				Some(employee) => match words.next() {
					Some("to") => match words.next() {
						Some(department) => Some(Command::Add {employee: employee.to_string(), department: department.to_string()}),
						None => None,
					},
					Some(_) => None,
					None => None,
				},
				None => None,
			},
			"quit" => Some(Command::Quit),
			_ => None,
		},
		None => None,
	}
}

fn execute_command(command: Command) {
	match command {
		Command::Add {employee, department} => println!("Add {} to {}", employee, department),
		Command::List {department: None} => println!("List employees of all departments"),
		Command::List {department: Some(department)} => println!("List employees of {}", department),
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

