#[derive(Debug)]
struct VisibleDrop<T> where T: std::fmt::Debug {
	x: T,
}

impl<T> VisibleDrop<T> where T: std::fmt::Debug {
	fn new(x: T) -> Self {
		VisibleDrop {
			x,
		}
	}
}

impl<T> Drop for VisibleDrop<T> where T: std::fmt::Debug {
	fn drop(self: &mut Self) {
		println!("Drop {:#?}!!!", self.x);
	}
}

#[derive(Debug)]
enum List<T> {
	Cons(T, std::cell::RefCell<std::rc::Rc<List<T>>>),
	Nil,
}

impl<T> List<T> {
	fn cons(list: std::cell::RefCell<std::rc::Rc<Self>>, x: T) -> std::cell::RefCell<std::rc::Rc<Self>> {
		std::cell::RefCell::new(std::rc::Rc::new(Self::Cons(x, list)))
	}

	fn cons_branch(list: &std::cell::RefCell<std::rc::Rc<Self>>, x: T) -> std::cell::RefCell<std::rc::Rc<Self>> {
		std::cell::RefCell::new(std::rc::Rc::new(Self::Cons(x, std::cell::RefCell::new(std::rc::Rc::clone(&list.borrow())))))
	}

	fn new(v: Vec<T>) -> std::cell::RefCell<std::rc::Rc<Self>> {
		let mut list: std::cell::RefCell<std::rc::Rc<Self>> = std::cell::RefCell::new(std::rc::Rc::new(Self::Nil));
		for x in v.into_iter().rev() {
			list = Self::cons(list, x);
		}
		list
	}

	fn tail(self: &Self) -> Option<&std::cell::RefCell<std::rc::Rc<Self>>> {
		match *self {
			Self::Cons(_, ref item) => Some(item),
			Self::Nil => None,
		}
	}
}

fn main() {
	let a: std::cell::RefCell<std::rc::Rc<List<VisibleDrop<i32>>>> = List::new(vec![VisibleDrop::new(5)]);
	println!("a = {:#?}", a);
	println!("a rc count = {}", std::rc::Rc::strong_count(&a.borrow()));
	let b: std::cell::RefCell<std::rc::Rc<List<VisibleDrop<i32>>>> = List::cons_branch(&a, VisibleDrop::new(10));
	println!("b = {:#?}", b);
	println!("a rc count = {}", std::rc::Rc::strong_count(&a.borrow()));
	println!("b rc count = {}", std::rc::Rc::strong_count(&b.borrow()));
	if let Some(link) = a.borrow().tail() {
		*link.borrow_mut() = std::rc::Rc::clone(&b.borrow());
	};
	println!("a rc count = {}", std::rc::Rc::strong_count(&a.borrow()));
	println!("b rc count = {}", std::rc::Rc::strong_count(&b.borrow()));
}

