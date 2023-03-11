use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::fs;

fn main() {
	/* Call a panic
	panic!("crash and burn");
	*/    

	/* Create a panic
	let v = vec![1, 2, 3];
	v[99];
	*/

	/* Handling errors
	let greeting_file_result = File::open("hello.txt");
	let greeting_file = match greeting_file_result {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}", e),
			},
			other_error => {
				panic!("Problem opening the file: {:?}", other_error);
			}
		},
	};
	*/

	/* Same as above, but cleaner to read with closures
	let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| {
				panic!("Problem creating the file: {:?}", error);
			})
		} else {
			panic!("Problem opening the file: {:?}", error);
		}
	});
	*/

	/* expect does the same as unwrap, but we can make our own error msg	
	let greeting_file = File::open("hello.txt")
		.expect("hello.txt should be included in this project");
	*/
	
	/* 
	Error propogation: if an error occurs in the read_username_from_file()
	function, it will propogate the error back to this main function to 
	handle.
	*/
	let username = match read_username_from_file() {
		Ok(s) => s,
		Err(_) => String::from("Default"),
	};
	println!("Username: {username}");

	// Propogation on a function that returns Option type
	match last_char_of_first_line("Hello") {
		Some(c) => println!("Char: {c}"),
		None => println!("No char returned"),
	};
}


/* Lengthy form of error propogation example
fn read_username_from_file() -> Result<String, io::Error> {
	let username_file_result = File::open("hello.txt");

	let mut username_file = match username_file_result {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut username = String::new();

	match username_file.read_to_string(&mut username) {
		Ok(_) => Ok(username)
		Err(e) => Err(e),
	}
}
*/

/* 
Cleaner version of error propogation example using "?". The "?" after a 
returned Result, if Ok, use the value inside Ok, otherwise if Err,
then the entire Err gets returned out of the function. 

fn read_username_from_file() -> Result<String, io::Error> {
	let mut username_file = File::open("hello.txt")?;
	let mut username = String::new();
	username_file.read_to_string(&mut username)?;
	Ok(username)
}
*/

/* Even cleaner version of error propogation with "?"
fn read_username_from_file() -> Result<String, io::Error> {
	let mut username = String::new();

	File::open("hello.txt")?.read_to_string(&mut username)?;

	Ok(username)
}
*/

// Cleanest version, although not a good example to teach error prop through
fn read_username_from_file() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}

// Propogation with Option type
fn last_char_of_first_line(text: &str) -> Option<char> {
	text.lines().next()?.chars().last()
}
