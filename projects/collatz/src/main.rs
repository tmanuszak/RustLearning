/// Find the first collatz number with a path length of input.
/// Print -1 if it doesnt exist.
use std::io;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

fn main() {
    'get_input: loop {
        let mut input = String::new();

        println!("Enter path length you want to find, or 'q' to quit: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let q = String::from("q");

        match input.trim().cmp(&q) {
            Ordering::Equal => break 'get_input,
            _ => (),
        }

        let input: u32 = input.trim().parse().expect("Please type a number!");

        let mut start: u128 = 2;
        let mut length_found = false;
        let mut smallest: u128 = u128::MAX;

        let mut start_length = HashMap::new();
        start_length.insert(1, 1);

        if input == 1 {
            length_found = true;
        }

        // While we havent found the answer
        while !length_found || start < smallest {
            // if the start path length hasnt been found
            if !start_length.contains_key(&start) {
                let mut d = VecDeque::new();
                d.push_back(start);
                let mut next: u128 = if start % 2 == 0 {
                    start / 2
                } else {
                    3 * start + 1
                };

                // while we dont know the path length of the next value
                while !start_length.contains_key(&next) {
                    d.push_back(next);
                    next = if next % 2 == 0 {
                        next / 2
                    } else {
                        3 * next + 1
                    };
                }

                let mut length = match start_length.get(&next) {
					Some(value) => *value,
					None => 0,
				}; 
				for value in d.iter().rev() {
                    length += 1;
                    if length == input {
                        length_found = true;
                        if *value < smallest {
                            smallest = *value;
                        }
                    }
                    start_length.insert(*value, length);
                }
            }
            start += 1;
        }

        println!("{smallest}");
    }
}
