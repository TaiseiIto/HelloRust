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
}

