#[derive(Debug)]
enum List<T> {
	Cons(T, std::cell::RefCell<std::rc::Rc<List<T>>>),
	Nil,
}

impl<T> List<T> {
	fn cons(list: std::cell::RefCell<std::rc::Rc<List<T>>>, x: T) -> std::cell::RefCell<std::rc::Rc<List<T>>> {
		std::cell::RefCell::new(std::rc::Rc::new(Self::Cons(x, list)))
	}

	fn new(v: Vec<T>) -> std::cell::RefCell<std::rc::Rc<List<T>>> {
		let mut list: std::cell::RefCell<std::rc::Rc<List<T>>> = std::cell::RefCell::new(std::rc::Rc::new(Self::Nil));
		for x in v.into_iter().rev() {
			list = Self::cons(list, x);
		}
		list
	}

	fn tail(self: &Self) -> Option<&std::cell::RefCell<std::rc::Rc<List<T>>>> {
		match *self {
			Self::Cons(_, ref item) => Some(item),
			Self::Nil => None,
		}
	}
}

fn main() {
	let a: std::cell::RefCell<std::rc::Rc<List<i32>>> = List::new(vec![5]);
	println!("a = {:#?}", a);
}

