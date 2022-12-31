fn main() {
	let mutex: std::sync::Mutex<i32> = std::sync::Mutex::new(5);
	{
		let mut num: std::sync::MutexGuard<i32> = mutex.lock().unwrap();
		*num = 6;
	}
	let num: std::sync::MutexGuard<i32> = mutex.lock().unwrap();
	println!("*num = {:#?}", *num);
	let counter: std::sync::Arc<std::sync::Mutex<i32>> = std::sync::Arc::new(std::sync::Mutex::new(0));
	let handles: Vec<std::thread::JoinHandle<()>> = (0..10).map(|_| {
		let counter: std::sync::Arc<std::sync::Mutex<i32>> = std::sync::Arc::clone(&counter);
		std::thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
		})
	}).collect::<Vec<std::thread::JoinHandle<()>>>();
	for handle in handles {
		handle.join().unwrap();
	}
	println!("Result: {}", *counter.lock().unwrap());
}

