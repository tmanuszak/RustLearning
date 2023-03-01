fn main() {
    println!("{}", first_word(&String::from("trey is cool")));
}

// Returns a slice of the first word in s
fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	&s[..]
}
