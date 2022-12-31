fn main() {
	let (tx, rx) = std::sync::mpsc::channel();
	let thread: std::thread::JoinHandle<()> = std::thread::spawn(move || {
		let val: String = "hi".to_string();
		tx.send(val).unwrap();
	});
	let received: String = rx.recv().unwrap();
	println!("Got: {}", received);
	thread.join().unwrap();
}

