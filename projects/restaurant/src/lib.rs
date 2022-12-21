mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
		pub fn seat_at_table() {}
	}
	pub mod serving {
		pub fn take_order() {}
		pub fn serve_order() {}
		pub fn take_payment() {}
	}
}

fn serve_order() {}

mod back_of_house {
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
}

pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
	// Absolute path
	hosting::add_to_waitlist();
	hosting::seat_at_table();
	// Relative path
	front_of_house::serving::take_order();
	front_of_house::serving::serve_order();
	front_of_house::serving::take_payment();
	back_of_house::fix_incorrect_order();
	let mut meal = back_of_house::Breakfast::summer("Rye");
	println!("meal = {:#?}", meal);
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please.", meal.toast);
	println!("meal = {:#?}", meal);
	println!("meal.get_seasonal_fruit() = {}", meal.get_seasonal_fruit());
	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;
	println!("order1 = {:#?}", order1);
	println!("order2 = {:#?}", order2);
}

