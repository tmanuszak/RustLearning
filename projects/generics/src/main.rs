// Generics in functions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];
	
	for item in list {
		if item > largest {
			largest = item;
		}
	}

	largest
}

// Generics in structs
struct Point<T> {
	x: T,
	y: T,
}

// Generics in method implementations
impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

// Generic method implementation constraint
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

// Generics in enums
enum Option<T> {
	Some(T),
	None,
}

fn main() {
	// Generics in functions example
    let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("The largest number is {}", result);
	
	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest(&char_list);
	println!("The largest char is {}", result);

	// Generics in structs example
	let integer = Point { x: 5, y: 10 };
	let float = Point { x: 1.0, y: 4.0 };
	println!("integer.x = {}", integer.x());
	println!("Distance from origin: {}", float.distance_from_origin());

}
