fn main() {
	let (tx_a, rx) = std::sync::mpsc::channel();
	let thread_a: std::thread::JoinHandle<()> = std::thread::spawn(move || {
		let val: String = "hi".to_string();
		tx_a.send(val).unwrap();
	});
	let received: String = rx.recv().unwrap();
	println!("Got: {}", received);
	let (tx_a, rx) = std::sync::mpsc::channel();
	let tx_b = std::sync::mpsc::Sender::clone(&tx_a);
	let thread_b: std::thread::JoinHandle<()> = std::thread::spawn(move || {
		let vals: Vec<String> = vec![
			"hi",
			"from",
			"the",
			"thread",
		].iter().map(|x| String::from(*x)).collect::<Vec<String>>();
		for val in vals {
			tx_a.send(val).unwrap();
			std::thread::sleep(std::time::Duration::from_secs(1));
		}
	});
	let thread_c: std::thread::JoinHandle<()> = std::thread::spawn(move || {
		let vals: Vec<String> = vec![
			"more",
			"messages",
			"for",
			"you",
		].iter().map(|x| String::from(*x)).collect::<Vec<String>>();
		for val in vals {
			tx_b.send(val).unwrap();
			std::thread::sleep(std::time::Duration::from_secs(1));
		}
	});
	for received in rx {
		println!("Got: {}", received);
	}
	thread_a.join().unwrap();
	thread_b.join().unwrap();
	thread_c.join().unwrap();
}

