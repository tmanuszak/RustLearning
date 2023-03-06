mod front_of_house;
mod back_of_house;

// Absolute path
pub use crate::front_of_house::hosting;
pub use crate::back_of_house::{Appetizer, Breakfast};

// Relative path
// pub use front_of_house::hosting;
// pub use back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
	hosting::add_to_waitlist();

	// Order a breakfast in the summer with Rye toast
	let mut meal = Breakfast::summer("Rye");
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please!", meal.toast);

	// The next line will not compile if uncommented; we are not allowed
	// to see or modify the seasonal fruit that comes with the meal
	// meal.seasonal_fruit = String::from("blueberries");

	let order1 = Appetizer::Soup;
	let order1 = Appetizer::Salad;
}

fn deliver_order() {}
