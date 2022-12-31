extern crate book_17_2;
use book_17_2::*;

#[derive(Debug)]
struct SelectBox {
	width: u32,
	height: u32,
	options: Vec<String>,
}

impl Draw for SelectBox {
	fn draw(&self) {
		println!("Draw {:#?}", self);
	}
}

fn main() {
	let screen = Screen {
		components: vec![
			Box::new(SelectBox {
				width: 75,
				height: 10,
				options: [
					"Yes",
					"Maybe",
					"No",
				].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
			}),
			Box::new(Button {
				width: 50,
				height: 10,
				label: "OK".to_string(),
			}),
		],
	};
	screen.run();
}

