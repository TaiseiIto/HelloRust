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
			"list" => match words.next() {
				Some(department) => Some(Command::List {department: Some(department.to_string())}),
				None => Some(Command::List {department: None}),
			}
			"quit" => Some(Command::Quit),
			_ => None,
		},
		None => None,
	}
}

fn execute_command(command: Command, organization: &mut std::collections::HashMap<String, Vec<String>>) -> bool {
	match command {
		Command::Add {employee, department} => {
			println!("Add {} to {}", employee, department);
			let employees: &mut Vec<String> = organization.entry(department).or_insert(Vec::new());
			employees.push(employee);
			true
		},
		Command::List {department: None} => {
			println!("List employees of all departments");
			let mut departments: Vec<&String> = organization.keys().collect::<Vec<&String>>();
			departments.sort();
			for department in departments {
				execute_command(Command::List {department: Some(department.to_string())}, &mut organization.clone());
			}
			true
		},
		Command::List {department: Some(department)} => {
			println!("List employees of {}", department);
			if let Some(employees) = organization.get(&department) {
				let mut employees: Vec<String> = employees.clone();
				employees.sort();
				for employee in employees {
					println!("\t{}", employee);
				}
			}
			true
		},
		Command::Quit => {
			println!("quit");
			false
		},
	}
}

fn interact(organization: &mut std::collections::HashMap<String, Vec<String>>) -> bool {
	let mut command: String = String::new();
	print!("manage employees > ");
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut command).expect("Failed to read line");
	match analyse_command(&*command) {
		Some(command) => execute_command(command, organization),
		None => true,
	}
}

fn main() {
	let mut organization: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
	let mut continuation: bool = true;
	while continuation {
		continuation = interact(&mut organization);
	}
}

