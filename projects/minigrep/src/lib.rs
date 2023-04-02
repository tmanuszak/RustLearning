use std::error::Error;
use std::{fs, env};

pub struct Config {
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool,
}

impl Config {
	// Returning a Config struct in Ok, or a static string wrapped around Err
	pub fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguents.");
		}
		
		// The parameters are immutable borrows. 
		// To instantiate the struct below, the struct needs to take ownership of
		// a value, which we can't do unless we clone it.
		let query = args[1].clone(); 
		let file_path = args[2].clone();

		// "var" returns Ok() with contents of the value of the environment 
		// variable, otherwise it will return Err() if the environment variable
		// does not exist. "is_ok" returns true if Ok() and returns false if 
		// Err().
		// EXAMPLE: $  IGNORE_CASE=1 cargo run -- to poem.txt
		let ignore_case = env::var("IGNORE_CASE").is_ok();

		Ok(Config { 
			query, 
			file_path,
			ignore_case,
		})
	}
}

// Returning nothing, or any struct that implements "Error" trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};

	for line in results {
		println!("{line}");
	}

	Ok(())
}

// We use lifetimes so that the compiler knows which string slice the returned
// value will be referencing.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

pub fn search_case_insensitive<'a>(
	query: &str, 
	contents: &'a str
) -> Vec<&'a str> {

	// query is shadowed variable. It will be of type String, not str
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		// Need to add '&' because contains takes str types, not String
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		// The "\" tells Rust not to put a newline at the beginning of the
		// string literal.
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		
		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}	
}
