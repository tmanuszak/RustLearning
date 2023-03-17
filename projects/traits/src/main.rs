use traits::{Summary, Tweet, TweetType, notify};

fn main() {
	let tweet = Tweet::new(
		&String::from("horse_ebooks"),
		&String::from(
			"of course, as you probably already know, people",
		),
		TweetType::Tweet,
		None,
	).expect("tweet not initialized");

	println!("1 new tweet: {}", tweet.summarize());
	notify(&tweet);
}
