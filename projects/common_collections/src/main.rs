use std::collections::HashMap;

fn main() {
	// -----------  VECTORS  --------------- 
    // Create new mutable empty vector that holds i32s 
	let mut v1: Vec<i32> = Vec::new(); 
	
	// Create new instantiated vector of i32s with macro 
	let v2 = vec![1, 2, 3]; 

	// Adding to the end of a vector
	v1.push(1);
	v1.push(2);
	v1.push(3);

	// Referencing and reading elements of a vector
	let third: &i32 = &v1[2];
	println!("The third element is {third}.");

	match v1.get(2) {
		Some(third) => println!("The third element is {third}."),
		None => println!("There is no third element."),
	}

	// Looping over each item in a vector
	for i in &v2 {
		println!("{i}");
	}

	for i in &mut v1 {
		*i += 1;
		println!("{i}");
	}

	// Using enums to let a vector store multiple types
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

	for i in &row {
		match i {
			SpreadsheetCell::Int(num) => println!("{num}"),
			SpreadsheetCell::Text(string) => println!("{string}"),
			SpreadsheetCell::Float(flt) => println!("{flt}"),
		}
	}

	// Dropping vectors
	{
		let _v = vec![1, 2, 3];
		// Do stuff with v
	}  // v goes out of stuff and itself with its elements are freed	

	// -----------  STRINGS  --------------- 
	// Creating a new emtpy String
	let mut _s = String::new();

	// Creating a String from a str literal
	let _s = "initial contents".to_string();

	// String concatenation
	let mut s = String::from("foo");
	s.push_str("bar");
	println!("{s}");

	// String concatenation with +. Here, the "add" function takes ownership
	// of s1, appends a copy of the contents of s2, and the returns ownership
	// of the result. Since "add" took ownership of s1, s1 is not usable 
	// hereafter.
	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = s1 + &s2;
	println!("{s3}");

	// String concatenation with format macro for difficult adds
	// This macro also does not take any ownership of the variables, so
	// they are usable after.
	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");
	let s = format!("{s1}-{s2}-{s3}");
	println!("{s}");	

	// -----------  HASH MAPS  --------------- 
	// For types that do not implement the Copy trait 
	// (i32 does, String does not), the hash map takes ownership of it. So,
	// field name will not be usable after the insert below.
	let mut scores = HashMap::new();
	let field_name = String::from("Blue");
	scores.insert(field_name, 10);
	scores.insert(String::from("Yellow"), 50);

	// get returns Option<&i32>. copied returns Option<i32>.
	// unwrap_or(0) returns the value if present, or 0 otherwise.
	let team_name = String::from("Blue");
	let _score = scores.get(&team_name).copied().unwrap_or(0);	

	// If "red" entry does not exist, insert 50
	scores.entry(String::from("red")).or_insert(50);
	
}
