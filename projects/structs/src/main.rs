struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
	User {
		username,
		email,
		active: true,
		sign_in_count: 1,
	}
}

fn main() {
	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};
	println!("user1.email = {}", user1.email);
	println!("user1.username = {}", user1.username);
	println!("user1.active = {}", user1.active);
	println!("user1.sign_in_count = {}", user1.sign_in_count);
	let user2 = build_user(String::from("me@example.com"), String::from("It's me."));
	println!("user2.email = {}", user2.email);
	println!("user2.username = {}", user2.username);
	println!("user2.active = {}", user2.active);
	println!("user2.sign_in_count = {}", user2.sign_in_count);
	let user3 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		..user1
	};
	println!("user3.email = {}", user3.email);
	println!("user3.username = {}", user3.username);
	println!("user3.active = {}", user3.active);
	println!("user3.sign_in_count = {}", user3.sign_in_count);
	let black = Color(0, 1, 2);
	println!("black.0 = {}", black.0);
	println!("black.1 = {}", black.1);
	println!("black.2 = {}", black.2);
	let origin = Point(3, 4, 5);
	println!("origin.0 = {}", origin.0);
	println!("origin.1 = {}", origin.1);
	println!("origin.2 = {}", origin.2);
}

