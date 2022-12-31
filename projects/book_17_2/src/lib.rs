pub trait Draw {
	fn draw(&self);
}

impl std::fmt::Debug for dyn Draw {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Draw").finish()
	}
}

#[derive(Debug)]
pub struct Screen {
	pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
	pub fn run(&self) {
		self.components.iter().map(|component| component.draw());
	}
}

#[derive(Debug)]
pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}

impl Draw for Button {
	fn draw(&self) {
		println!("Draw {:#?}", self);
	}
}

