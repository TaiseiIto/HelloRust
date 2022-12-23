use rand::Rng;

fn mean(sequence: &Vec<i8>) -> f64 {
	let mut sum: i32 = 0;
	for n in sequence {
		sum += *n as i32;
	}
	sum as f64 / sequence.len() as f64
}

fn median(sequence: &Vec<i8>) -> i8 {
	let mut sorted: Vec<i8> = sequence.clone();
	sorted.sort();
	sorted[sorted.len() / 2]
}

fn mode(sequence: &Vec<i8>) -> i8 {
	let mut value2count: std::collections::HashMap<i8, u32> = std::collections::HashMap::new();
	for n in sequence {
		*value2count.entry(*n).or_insert(0) += 1;
	}
	let mut max_count: u32 = 0;
	let mut mode: i8 = 0;
	for (value, count) in value2count {
		if max_count < count {
			max_count = count;
			mode = value;
		}
	}
	mode
}

fn main() {
	let mut rng = rand::thread_rng();
	let mut sequence: Vec<i8> = Vec::new();
	for _ in 0..100 {
		sequence.push(rng.gen());
	}
	println!("sequence = {:?}", sequence);
	println!("mean(sequence) = {:?}", mean(&sequence));
	println!("median(sequence) = {:?}", median(&sequence));
	println!("mode(sequence) = {:?}", mode(&sequence));
}

