use std::{env, process};
use minigrep::Config;

fn main() {
	let args: Vec<String> = env::args().collect();

	// "unwrap_or_else" will return the contents inside Ok(), or take the 
	// contents inside the Err() case and use it as input (err) to the anonymous 
	// function.	
	let config = Config::build(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {err}");
		process::exit(1);
	});

	println!("Searching for {}", config.query);
	println!("In file {}", config.file_path);

	// If an error is returned from run, handle it.
	// We use "if let" instead of "unwrap_or_else" because we dont care about
	// unwrapping the Ok(()) case, we only want to handle an error.
	if let Err(e) = minigrep::run(config) {
		println!("Application error: {e}");
		process::exit(1);
	}
}