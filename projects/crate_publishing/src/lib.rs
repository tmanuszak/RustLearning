//! # Crate Publishing
//!
//! `crate_publishing` is a collection of example code for documentation and 
//! publishing a public crate.

// The above "//!" lines are used to describe the contents of the file

// A documentation comment is three slashes.
// To build and open the HTML docs, run "cargo doc --open".

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crate_publishing::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub use self::kinds::PrimaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
	pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
	pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    	println!("{:?} + {:?} = Orange!", c1, c2);
		SecondaryColor::Orange
	}
}
