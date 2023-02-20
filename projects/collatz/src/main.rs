use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
/// Find the first collatz number with a path length of input.
/// Print 0 if it doesnt exist.
use std::io;

fn main() {
    'get_input: loop {
        let mut input = String::new();

        println!("Enter path length you want to find, or 'q' to quit: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // If no input, continue loop
        if input.trim().len() == 0 {
            continue;
        }

        // See if we quit
        let q = String::from("q");
        match input.trim().cmp(&q) {
            Ordering::Equal => break 'get_input,
            _ => (),
        }

        // Parse input into u32
        let input: u32 = input.trim().parse().expect("Please type a number!");

        println!("{}", collatz(input));
    }
}

fn collatz(input: u32) -> u128 {
    if input < 2 {
        return input.into();
    }

    // Starting to find the result
    // Variables
    let mut start: u128 = 2; // start of the path
    let mut length_found = false; // if we have found the length
    let mut smallest: u128 = 2u128.saturating_pow(input); // the smallest the answer could be

    // Hash map where start -> length
    let mut start_length = HashMap::new();
    start_length.insert(1u128, 1u32);

    // Stores the path of the current working iteration
    let mut d = VecDeque::new();
    let mut len: u32 = 0; // length of the queue, since .len() -> usize

    // While we havent found the answer
    while start < smallest {
        // if the start path length hasnt been found
        if !start_length.contains_key(&start) {
            d.clear();
            d.push_back(start);
            len = 1;
            let mut next: u128 = if start % 2 == 0 {
                start / 2
            } else {
                let (mul, of1) = start.overflowing_mul(3);
                let (res, of2) = mul.overflowing_add(1);
                if of1 || of2 {
                    println!("Overflowing operation with start: {start}");
                    return 0;
                }
                res
            };

            // while we dont know the path length of the next value
            while !start_length.contains_key(&next) {
                d.push_back(next);
                len += 1;
                next = if next % 2 == 0 {
                    next / 2
                } else {
                    let (mul, of1) = next.overflowing_mul(3);
                    let (res, of2) = mul.overflowing_add(1);
                    if of1 || of2 {
                        println!("Overflowing operation with start: {start}");
                        return 0;
                    }
                    res
                };
            }

            len = match start_length.get(&next) {
                Some(value) => *value + len,
                None => 0, // Shouldnt ever be the case since we broke above
            };
            for value in d.iter() {
                if len == input {
                    length_found = true;
                    if *value < smallest {
                        smallest = *value;
                    }
                }
                start_length.insert(*value, len);
                len -= 1;
            }
        }
        start += 1;
    }

    if !length_found {
        return 0;
    } else {
        return smallest;
    }
}
