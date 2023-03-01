#[derive(Debug)]
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	// METHODS
	// "&self" is short for "self: &Self"
	fn area(&self) -> u32 {
		self.width * self.height
	}

	// self can hold another rectangle inside of it
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	// ASSOCIATED FUNCTIONS
	// "Self" is an alias for the struct being implemented, "Rectangle"
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}

fn main() {
	let user1 = User {
		active: true,
		username: String::from("someusername123"),
		email: String::from("someone@example.com"),
		sign_in_count: 1,
	};
	println!("Active: {}, Username: {}, Email: {}, Sign In Count: {}",
		user1.active,
		user1.username,
		user1.email,
		user1.sign_in_count
	);
	
	let user2 = build_user("trey@mail.com".to_string(), "Trey".to_string());
	println!("User 2 is {:#?}",
		user2
	); // Need #[derive(Debug)] to do this

	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};
	let rect2 = Rectangle::square(10);
	println!("Area is {}", rect1.area());
	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username, // shorthand initialization
		email,
		sign_in_count: 1,
	}
}
