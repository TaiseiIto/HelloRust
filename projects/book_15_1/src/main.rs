#[derive(Debug)]
enum List<T> {
	Cons(T, Box<List<T>>),
	Nil,
}

impl<T> List<T> {
	fn new(_: &Vec<T>) -> List<T> {
		List::Nil
	}
}

fn main() {
	let b: Box<i32> = Box::new(5);
    println!("b = {}", b);
	let list: List<i32> = List::new(&vec![1,2,3]);
	println!("list = {:#?}", list);
}

