/*
Structs for various kinds of text. NewsArticle and Tweet, for example. Tweets
have constraints in text length. We create a news aggregator here that can
display summaries of text by calling a "summarize" method on an instance. We
use this example to showcase traits.
*/

pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl NewsArticle {
    pub fn new(headline: &str, location: &str, author: &str, content: &str) -> Self {
        Self {
            headline: headline.to_string(),
            location: location.to_string(),
            author: author.to_string(),
            content: content.to_string(),
        }
    }
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub enum TweetType {
    Tweet,
    Retweet,
    Reply,
}

pub struct Tweet {
    username: String,
    content: String,
    tweet_type: TweetType,
    replied_to: Option<u32>, // uid of tweet being replied to
}

impl Tweet {
    pub fn new(
        username: &str,
        content: &str,
        tweet_type: TweetType,
        replied_to: Option<u32>,
    ) -> Result<Self, String> {
        if content.len() > 280 {
            return Err(String::from("Tweet too long."));
        } else if let TweetType::Reply = tweet_type {
            if replied_to.is_none() {
                return Err(String::from("Tweet is a reply to nothing."));
            }
        }

        Ok(Self {
            username: username.to_string(),
            content: content.to_string(),
            tweet_type,
            replied_to,
        })
    }
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

pub trait Summary {
    // Default behavior
	fn summarize(&self) -> String {
		String::from("(Read more...)")
	}
}

// A function that takes a trait as a parameter. Only types that implement the
// trait can call this function.
pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}
