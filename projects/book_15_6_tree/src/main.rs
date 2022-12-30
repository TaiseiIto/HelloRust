#[derive(Debug)]
struct Node<T> {
	value: T,
	parent: std::cell::RefCell<std::rc::Weak<Node<T>>>,
	children: std::cell::RefCell<Vec<std::rc::Rc<Node<T>>>>,
}

impl<T> Node<T> {
	fn new_leaf(x: T) -> std::rc::Rc<Self> {
		std::rc::Rc::new(Self {
			value: x,
			parent: std::cell::RefCell::new(std::rc::Weak::new()),
			children: std::cell::RefCell::new(vec![]),
		})
	}
}

fn main() {
	let leaf: std::rc::Rc<Node<i32>> = Node::new_leaf(3);
	let branch: std::rc::Rc<Node<i32>> = std::rc::Rc::new(Node {
		value: 5,
		parent: std::cell::RefCell::new(std::rc::Weak::new()),
		children: std::cell::RefCell::new(vec![std::rc::Rc::clone(&leaf)]),
	});
	println!("leaf = {:#?}", leaf);
	println!("branch = {:#?}", branch);
}

