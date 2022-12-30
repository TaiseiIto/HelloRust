#[derive(Debug)]
enum List<T> {
	Cons(T, std::rc::Rc<List<T>>),
	Nil,
}

impl<T> List<T> {
	fn cons(list: &std::rc::Rc<List<T>>, x: T) -> std::rc::Rc<List<T>> {
		std::rc::Rc::new(List::Cons(x, std::rc::Rc::clone(list)))
	}

	fn new(v: Vec<T>) -> std::rc::Rc<List<T>> {
		let mut list: std::rc::Rc<List<T>> = std::rc::Rc::new(List::Nil);
		for i in v.into_iter().rev() {
			list = Self::cons(&list, i);
		}
		list
	}
}

fn main() {
	let a: std::rc::Rc<List<i32>> = List::new(vec![5, 10]);
	println!("a = {:#?}", a);
	println!("count = {}", std::rc::Rc::strong_count(&a));
	let b: std::rc::Rc<List<i32>> = List::cons(&a, 3);
	println!("b = {:#?}", b);
	println!("count = {}", std::rc::Rc::strong_count(&a));
	let c: std::rc::Rc<List<i32>> = List::cons(&a, 4);
	println!("c = {:#?}", c);
	println!("count = {}", std::rc::Rc::strong_count(&a));
}

