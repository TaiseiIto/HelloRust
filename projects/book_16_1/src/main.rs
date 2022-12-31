fn main() {
	let thread_a: std::thread::JoinHandle<()> = std::thread::spawn(|| {
		for i in 1..10 {
			println!("Hi! number {} from the spawned thread!", i);
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	});
	let vector_a: Vec<i32> = vec![1, 2, 3];
	let vector_b: Vec<i32> = vec![4, 5, 6];
	let thread_b: std::thread::JoinHandle<()> = std::thread::spawn(move || {
		println!("vector_a = {:?}", vector_a);
	});
	for i in 1..10 {
			println!("Hi! number {} from the main thread!", i);
			std::thread::sleep(std::time::Duration::from_millis(1));
	}
	println!("vector_b = {:?}", vector_b);
	thread_a.join().unwrap();
	thread_b.join().unwrap();
}

