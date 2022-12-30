#[derive(Debug)]
enum List<T> {
	Cons(T, std::cell::RefCell<std::rc::Rc<List<T>>>),
	Nil,
}

impl<T> List<T> {
	fn tail(self: &Self) -> Option<&std::cell::RefCell<std::rc::Rc<List<T>>>> {
		match *self {
			Self::Cons(_, ref item) => Some(item),
			Self::Nil => None,
		}
	}
}

fn main() {
    println!("Hello, world!");
}
