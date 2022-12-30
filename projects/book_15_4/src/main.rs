#[derive(Debug)]
enum List<T> {
	Cons(T, std::rc::Rc<List<T>>),
	Nil,
}

impl<T> List<T> {
	fn cons(self: Self, x: T) -> List<T> {
		List::Cons(x, std::rc::Rc::new(self))
	}

	fn new(v: Vec<T>) -> List<T> {
		let mut list: List<T> = List::Nil;
		for i in v.into_iter().rev() {
			list = list.cons(i);
		}
		list
	}
}

fn main() {
	let a: List<i32> = List::new(vec![5, 10]);
	println!("a = {:#?}", a);
}

