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

	fn add_parent(parent: T, child: &std::rc::Rc<Self>) -> std::rc::Rc<Self> {
		let parent_tree: std::rc::Rc<Self> = std::rc::Rc::new(Self {
			value: parent,
			parent: std::cell::RefCell::new(std::rc::Weak::new()),
			children: std::cell::RefCell::new(vec![std::rc::Rc::clone(child)]),
		});
		*child.parent.borrow_mut() = std::rc::Rc::downgrade(&parent_tree);
		parent_tree
	}
}

fn main() {
	let leaf: std::rc::Rc<Node<VisibleDrop<i32>>> = Node::new_leaf(VisibleDrop::new(3));
	println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
	println!("leaf strong count = {}", std::rc::Rc::strong_count(&leaf));
	println!("leaf weak count = {}", std::rc::Rc::weak_count(&leaf));
	let branch: std::rc::Rc<Node<VisibleDrop<i32>>> = Node::add_parent(VisibleDrop::new(5), &leaf);
	println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
	println!("branch strong count = {}", std::rc::Rc::strong_count(&branch));
	println!("branch weak count = {}", std::rc::Rc::weak_count(&branch));
	println!("leaf strong count = {}", std::rc::Rc::strong_count(&leaf));
	println!("leaf weak count = {}", std::rc::Rc::weak_count(&leaf));
}

