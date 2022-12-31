fn main() {
	std::thread::spawn(|| {
		for i in 1..10 {
			println!("Hi! number {} from the spawned thread!", i);
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	});
	for i in 1..10 {
			println!("Hi! number {} from the main thread!", i);
			std::thread::sleep(std::time::Duration::from_millis(1));
	}
}

