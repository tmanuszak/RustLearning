use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
	Red,
	Blue,
}

struct Inventory {
	shirts: Vec<ShirtColor>,
}

impl Inventory {
	fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
		user_preference.unwrap_or_else(|| self.most_stocked())
	}

	fn most_stocked(&self) -> ShirtColor {
		let mut num_red = 0;
		let mut num_blue = 0;

		for color in &self.shirts {
			match color {
				ShirtColor::Red => num_red += 1,
				ShirtColor::Blue => num_blue += 1,
			}
		}
		if num_red > num_blue {
			ShirtColor::Red
		} else {
			ShirtColor::Blue
		}
	}
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height:u32,
}

fn main() {
	// Example of closures
	let store = Inventory {
		shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
	};
	
	let user_pref1 = Some(ShirtColor::Red);
	let giveaway1 = store.giveaway(user_pref1);
	println!(
		"The user with preference {:?} gets {:?}",
		user_pref1, giveaway1
	);
	
	let user_pref2 = None;
	let giveaway2 = store.giveaway(user_pref2);
	println!(
		"The user with preference {:?} gets {:?}",
		user_pref2, giveaway2
	);

// ------------------------------------------------------------------------- //

	// Similaries and differences between functions and closures
	// function definition
	fn  add_one_v1   (x: u32) -> u32 { x + 1 }
	// closure with type annotations
	let add_one_v2 = |x: u32| -> u32 { x + 1 };
	// closure is evaluated at compile time because types will be inferred
	let add_one_v3 = |x|             { x + 1 };
	// Brackets are optional with 1 expression. Also evaluated at comp time.
	let add_one_v4 = |x|               x + 1  ;

	// code so the compiler knows the type to v3 and v4 above
	let x = add_one_v3(5);
	let x = add_one_v4(5);	

// ------------------------------------------------------------------------- //

	// Example of closure type inferred at compile time
	let example_closure = |x| x;
	let s = example_closure(String::from("hello"));
	// Uncommenting the below statement will make a compile time error because
	// the compiler first inferred the type of x to be a String, not an i32.
	// let n = example_closure(5);

// ------------------------------------------------------------------------- //

	// Example of different closure captures. Closures can capture values from their
	// environment in three ways, which directly map to the three ways a function 
	// can take a parameter: borrowing immutably, borrowing mutably, taking 
	// ownership.

	// --- Immutably borrow example --- //
	println!("\nImmutable borrow example");
	let list = vec![1, 2, 3];
	println!("Before defining closure: {:?}", list);
	
	// Compiler will know it only needs an immutable borrow in the body
	let only_borrows = || println!("From closure: {:?}", list);
	println!("Before calling closure: {:?}", list);
	only_borrows();
	println!("After calling closure: {:?}", list);

	// --- Mutably borrow example --- //
	println!("\nMutable borrow example");
	let mut list = vec![1, 2, 3];
	println!("Before defining closure: {:?}", list);
	
	// Compiler will know it needs to borrow list mutably
	let mut borrows_mutably = || list.push(7);

	// We cannot have a println here with list because the closure is mutably
	// borrowing the list right here and println needs an immutable borrow.
	
	borrows_mutably();
	// Now the mutable borrows are done and we can call a println on list
	println!("After calling closure: {:?}", list);

	// --- Taking ownership example --- //
	let list = vec![1, 2, 3];
	println!("Before defining closure: {:?}", list);

	// We spawn a new thread, giving the thread a closure to run as an argument.
	// The closure body prints out the list. The "move" keyword allows the 
	// thread to take ownership of the list. If we didnt allow it to take 
	// ownership and the main thread finished and deleted the list before the 
	// spawned thread was able to print it, there would be an invalid reference.
	thread::spawn(move || println!("From thread: {:?}", list))
		.join()
		.unwrap();

// -------------------------------------------------------------------------- //
	
	println!();
	let mut list = [
		Rectangle { width: 10, height: 1 },
		Rectangle { width: 3, height: 5 },
		Rectangle {width: 7, height: 12 },
	];

	list.sort_by_key(|r| r.width);
	println!("{:#?}", list);
}	
