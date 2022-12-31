fn main() {
	let (tx_a, rx) = std::sync::mpsc::channel();
	let thread_a: std::thread::JoinHandle<()> = std::thread::spawn(move || {
		let val: String = "hi".to_string();
		tx_a.send(val).unwrap();
	});
	let received: String = rx.recv().unwrap();
	println!("Got: {}", received);
	thread_a.join().unwrap();
}

