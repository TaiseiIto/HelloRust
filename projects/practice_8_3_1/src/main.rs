use rand::Rng;

fn mean(sequence: &Vec<i8>) -> f64 {
	let mut sum: i32 = 0;
	for n in sequence {
		sum += *n as i32;
	}
	sum as f64 / sequence.len() as f64
}

fn main() {
	let mut rng = rand::thread_rng();
	let mut sequence: Vec<i8> = Vec::new();
	for _ in 0..10 {
		sequence.push(rng.gen());
	}
	println!("sequence = {:?}", sequence);
	println!("mean(sequence) = {:?}", mean(&sequence));
}

