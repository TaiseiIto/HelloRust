#[derive(Debug)]
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}

#[derive(Debug)]
enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		match self {
			Message::Quit =>                 println!("call Quit"),
			Message::Move {x, y} =>          println!("call Move {{x: {}, y: {}}}", x, y),
			Message::Write(string) =>        println!("call Write(\"{}\")", string),
			Message::ChangeColor(r, g, b) => println!("call ChangeColor({}, {}, {})", r, g, b),
		}
	}
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	Arizona,
	Arkansas,
	California,
	Colorado,
	Connecticut,
	Delaware,
	Florida,
	Georgia,
	Hawaii,
	Idaho,
	Illinois,
	Indiana,
	Iowa,
	Kansas,
	Kentucky,
	Louisiana,
	Maine,
	Maryland,
	Massachusetts,
	Michigan,
	Minnesota,
	Mississippi,
	Missouri,
	Montana,
	Nebraska,
	Nevada,
	NewHampshire,
	NewJersey,
	NewMexico,
	NewYork,
	NorthCarolina,
	NorthDakota,
	Ohio,
	Oklahoma,
	Oregon,
	Pennsylvania,
	RhodeIsland,
	SouthCarolina,
	SouthDakota,
	Tennessee,
	Texas,
	Utah,
	Vermont,
	Virginia,
	Washington,
	WestVirginia,
	Wisconsin,
	Wyoming,
}

#[derive(Debug)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i32 {
	match coin {
		Coin::Penny => {
			println!("Lucky penny!");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:#?}!", state);
			25
		},
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		Some(i) => Some(i + i),
		None => None,
	}
}

fn print_value(x: u8) {
	match x {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		4 => println!("four"),
		_ => (),
	}
}

fn is_three(x: u8) {
	if let 3 = x {
		println!("three");
	} else {
		println!("not three");
	}
}

fn main() {
	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));
	println!("home = {:#?}", home);
	println!("loopback = {:#?}", loopback);
	let quit_message = Message::Quit;
	let move_message = Message::Move { x: 123, y: 456};
	let write_message = Message::Write(String::from("write message"));
	let change_color_message = Message::ChangeColor(987, 654, 321);
	println!("quit_message = {:#?}", quit_message);
	println!("move_message = {:#?}", move_message);
	println!("write_message = {:#?}", write_message);
	println!("change_color_message = {:#?}", change_color_message);
	quit_message.call();
	move_message.call();
	write_message.call();
	change_color_message.call();
	println!("value_in_cents(Coin::Penny) = {}", value_in_cents(Coin::Penny));
	println!("value_in_cents(Coin::Nickel) = {}", value_in_cents(Coin::Nickel));
	println!("value_in_cents(Coin::Dime) = {}", value_in_cents(Coin::Dime));
	println!("value_in_cents(Coin::Quarter(UsState::Alabama)) = {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
	println!("value_in_cents(Coin::Quarter(UsState::Alaska)) = {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
	println!("value_in_cents(Coin::Quarter(UsState::Arizona)) = {}", value_in_cents(Coin::Quarter(UsState::Arizona)));
	println!("value_in_cents(Coin::Quarter(UsState::Arkansas)) = {}", value_in_cents(Coin::Quarter(UsState::Arkansas)));
	println!("value_in_cents(Coin::Quarter(UsState::California)) = {}", value_in_cents(Coin::Quarter(UsState::California)));
	println!("value_in_cents(Coin::Quarter(UsState::Colorado)) = {}", value_in_cents(Coin::Quarter(UsState::Colorado)));
	println!("value_in_cents(Coin::Quarter(UsState::Connecticut)) = {}", value_in_cents(Coin::Quarter(UsState::Connecticut)));
	println!("value_in_cents(Coin::Quarter(UsState::Delaware)) = {}", value_in_cents(Coin::Quarter(UsState::Delaware)));
	println!("value_in_cents(Coin::Quarter(UsState::Florida)) = {}", value_in_cents(Coin::Quarter(UsState::Florida)));
	println!("value_in_cents(Coin::Quarter(UsState::Georgia)) = {}", value_in_cents(Coin::Quarter(UsState::Georgia)));
	println!("value_in_cents(Coin::Quarter(UsState::Hawaii)) = {}", value_in_cents(Coin::Quarter(UsState::Hawaii)));
	println!("value_in_cents(Coin::Quarter(UsState::Idaho)) = {}", value_in_cents(Coin::Quarter(UsState::Idaho)));
	println!("value_in_cents(Coin::Quarter(UsState::Illinois)) = {}", value_in_cents(Coin::Quarter(UsState::Illinois)));
	println!("value_in_cents(Coin::Quarter(UsState::Indiana)) = {}", value_in_cents(Coin::Quarter(UsState::Indiana)));
	println!("value_in_cents(Coin::Quarter(UsState::Iowa)) = {}", value_in_cents(Coin::Quarter(UsState::Iowa)));
	println!("value_in_cents(Coin::Quarter(UsState::Kansas)) = {}", value_in_cents(Coin::Quarter(UsState::Kansas)));
	println!("value_in_cents(Coin::Quarter(UsState::Kentucky)) = {}", value_in_cents(Coin::Quarter(UsState::Kentucky)));
	println!("value_in_cents(Coin::Quarter(UsState::Louisiana)) = {}", value_in_cents(Coin::Quarter(UsState::Louisiana)));
	println!("value_in_cents(Coin::Quarter(UsState::Maine)) = {}", value_in_cents(Coin::Quarter(UsState::Maine)));
	println!("value_in_cents(Coin::Quarter(UsState::Maryland)) = {}", value_in_cents(Coin::Quarter(UsState::Maryland)));
	println!("value_in_cents(Coin::Quarter(UsState::Massachusetts)) = {}", value_in_cents(Coin::Quarter(UsState::Massachusetts)));
	println!("value_in_cents(Coin::Quarter(UsState::Michigan)) = {}", value_in_cents(Coin::Quarter(UsState::Michigan)));
	println!("value_in_cents(Coin::Quarter(UsState::Minnesota)) = {}", value_in_cents(Coin::Quarter(UsState::Minnesota)));
	println!("value_in_cents(Coin::Quarter(UsState::Mississippi)) = {}", value_in_cents(Coin::Quarter(UsState::Mississippi)));
	println!("value_in_cents(Coin::Quarter(UsState::Missouri)) = {}", value_in_cents(Coin::Quarter(UsState::Missouri)));
	println!("value_in_cents(Coin::Quarter(UsState::Montana)) = {}", value_in_cents(Coin::Quarter(UsState::Montana)));
	println!("value_in_cents(Coin::Quarter(UsState::Nebraska)) = {}", value_in_cents(Coin::Quarter(UsState::Nebraska)));
	println!("value_in_cents(Coin::Quarter(UsState::Nevada)) = {}", value_in_cents(Coin::Quarter(UsState::Nevada)));
	println!("value_in_cents(Coin::Quarter(UsState::NewHampshire)) = {}", value_in_cents(Coin::Quarter(UsState::NewHampshire)));
	println!("value_in_cents(Coin::Quarter(UsState::NewJersey)) = {}", value_in_cents(Coin::Quarter(UsState::NewJersey)));
	println!("value_in_cents(Coin::Quarter(UsState::NewMexico)) = {}", value_in_cents(Coin::Quarter(UsState::NewMexico)));
	println!("value_in_cents(Coin::Quarter(UsState::NewYork)) = {}", value_in_cents(Coin::Quarter(UsState::NewYork)));
	println!("value_in_cents(Coin::Quarter(UsState::NorthCarolina)) = {}", value_in_cents(Coin::Quarter(UsState::NorthCarolina)));
	println!("value_in_cents(Coin::Quarter(UsState::NorthDakota)) = {}", value_in_cents(Coin::Quarter(UsState::NorthDakota)));
	println!("value_in_cents(Coin::Quarter(UsState::Ohio)) = {}", value_in_cents(Coin::Quarter(UsState::Ohio)));
	println!("value_in_cents(Coin::Quarter(UsState::Oklahoma)) = {}", value_in_cents(Coin::Quarter(UsState::Oklahoma)));
	println!("value_in_cents(Coin::Quarter(UsState::Oregon)) = {}", value_in_cents(Coin::Quarter(UsState::Oregon)));
	println!("value_in_cents(Coin::Quarter(UsState::Pennsylvania)) = {}", value_in_cents(Coin::Quarter(UsState::Pennsylvania)));
	println!("value_in_cents(Coin::Quarter(UsState::RhodeIsland)) = {}", value_in_cents(Coin::Quarter(UsState::RhodeIsland)));
	println!("value_in_cents(Coin::Quarter(UsState::SouthCarolina)) = {}", value_in_cents(Coin::Quarter(UsState::SouthCarolina)));
	println!("value_in_cents(Coin::Quarter(UsState::SouthDakota)) = {}", value_in_cents(Coin::Quarter(UsState::SouthDakota)));
	println!("value_in_cents(Coin::Quarter(UsState::Tennessee)) = {}", value_in_cents(Coin::Quarter(UsState::Tennessee)));
	println!("value_in_cents(Coin::Quarter(UsState::Texas)) = {}", value_in_cents(Coin::Quarter(UsState::Texas)));
	println!("value_in_cents(Coin::Quarter(UsState::Utah)) = {}", value_in_cents(Coin::Quarter(UsState::Utah)));
	println!("value_in_cents(Coin::Quarter(UsState::Vermont)) = {}", value_in_cents(Coin::Quarter(UsState::Vermont)));
	println!("value_in_cents(Coin::Quarter(UsState::Virginia)) = {}", value_in_cents(Coin::Quarter(UsState::Virginia)));
	println!("value_in_cents(Coin::Quarter(UsState::Washington)) = {}", value_in_cents(Coin::Quarter(UsState::Washington)));
	println!("value_in_cents(Coin::Quarter(UsState::WestVirginia)) = {}", value_in_cents(Coin::Quarter(UsState::WestVirginia)));
	println!("value_in_cents(Coin::Quarter(UsState::Wisconsin)) = {}", value_in_cents(Coin::Quarter(UsState::Wisconsin)));
	println!("value_in_cents(Coin::Quarter(UsState::Wyoming)) = {}", value_in_cents(Coin::Quarter(UsState::Wyoming)));
	println!("plus_one(None) = {:#?}", plus_one(None));
	println!("plus_one(Sone(5)) = {:#?}", plus_one(Some(5)));
	println!("print_value(1)");
	print_value(1);
	println!("print_value(2)");
	print_value(2);
	println!("print_value(3)");
	print_value(3);
	println!("print_value(4)");
	print_value(4);
	println!("print_value(5)");
	print_value(5);
	println!("is_three(1)");
	is_three(1);
	println!("is_three(2)");
	is_three(2);
	println!("is_three(3)");
	is_three(3);
	println!("is_three(4)");
	is_three(4);
	println!("is_three(5)");
	is_three(5);
}

