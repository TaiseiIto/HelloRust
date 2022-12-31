use book_17_1::*;

fn main() {
    let mut ac = AveragedCollection::new();
    println!("ac.average() = {}", ac.average());
	for i in 0..10 {
		ac.add(i);
	}
    println!("ac.average() = {}", ac.average());
	ac.remove();
    println!("ac.average() = {}", ac.average());
}

