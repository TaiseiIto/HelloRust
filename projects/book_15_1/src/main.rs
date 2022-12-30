#[derive(Debug)]
enum List<T> where T: Clone {
	Cons(T, Box<List<T>>),
	Nil,
}

impl<T> List<T> where T: Clone {
	fn cons(self, x: T) -> List<T> {
		List::Cons(x, Box::new(self))
	}

	fn new(v: Vec<T>) -> List<T> {
		let mut list: List<T> = List::Nil;
		for x in v.iter().rev() {
			list = list.cons(x.clone());
		}
		list
	}
}

fn main() {
	let b: Box<i32> = Box::new(5);
    println!("b = {}", b);
	let list: List<i32> = List::new(vec![1,2,3]);
	println!("list = {:#?}", list);
}

