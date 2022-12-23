use rand::Rng;

fn main() {
	let mut rng = rand::thread_rng();
	let x: u8 = rng.gen();
	println!("x = {}", x);
}

