// WATCH THIS! https://www.youtube.com/watch?v=juIINGuZyBc&ab_channel=Let%27sGetRusty

fn main() {
    /* This code wont compile since r is a dangling pointer. The lifetime of r,
	// Which is 'a references x, which has a shorter lifetime of 'b
	fn main() {
		let r;   -------------------+--'a
									|
		{                           |
			let x = 5;  ---+--'b    |
			r = &x         |        |
		}   ---------------+        |
		println!("r: {}", r);       |
	}  -----------------------------+
	*/

	// This compiles
	let x = 5;             // -----------+-- b'
	let r = &x;            // --+-- 'a   |
	println!("r: {}", r);  //   |        |
	                       // --+        |
                           // -----------+

	main2();

	let x = 5;
	let y = 6;
	let r = return_same(&x, &y);
	println!("{}", x);
	println!("{}", y);
	println!("{}", r);

	let mut s = String::from("Trey");
	let r = same(&s);
	println!("{} {}", s, r);
	s.push('!');
	println!("{}", s);
} 

fn main2() {
	let string1 = String::from("abcd");
	let string2 = "xyz";

	let result = longest(string1.as_str(), string2);
	println!("The longest string is {}", result);
}

/* Program does not compile with this funcion because the return type contains
// a borrowed value, but the signature does not say whether it is borrowed from
// x or y.
fn longest(x: &str, y: &str) -> &str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn return_same<'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
	if x % 2 == 0 {
		x
	}
	else {
		y
	}
}

fn same(s: &str) -> &str {
	s
}
