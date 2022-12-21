mod front_of_house;
mod back_of_house;

fn serve_order() {}

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

