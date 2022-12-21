#[derive(Debug)]
pub struct Breakfast {
	pub toast: String,
	seasonal_fruit: String,
}
impl Breakfast {
	pub fn summer(toast: &str) -> Breakfast {
		Breakfast {
			toast: String::from(toast),
			seasonal_fruit: String::from("peaches"),
		}
	}
	pub fn get_seasonal_fruit(&self) -> &String {
		&self.seasonal_fruit
	}
}
#[derive(Debug)]
pub enum Appetizer {
	Soup,
	Salad,
}
pub fn fix_incorrect_order() {
	cook_order();
	super::serve_order();
}
fn cook_order() {}

