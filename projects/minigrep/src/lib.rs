use std::error::Error;
use std::fs;

pub struct Config {
	pub query: String,
	pub file_path: String,
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

		Ok(Config { query, file_path })
	}
}

// Returning nothing, or any struct that implements "Error" trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	// "?" either unwraps the contents if Ok, or returns the error to the caller.
	let contents = fs::read_to_string(config.file_path)?;

	println!("With text:\n{contents}");

	Ok(())
}
