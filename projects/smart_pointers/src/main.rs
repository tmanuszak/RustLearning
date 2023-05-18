// ------------------------------------------------------------------------ //

// For recursive types example
enum List {
	// If we use List instead of Box<List>, we get a compile error because the
	// compiler thinks the List type size is infinite.
	// When we use Box<List> instead, the size of list is well defined since 
	// Box is a pointer.
	Cons(i32, Box<List>),  
	Nil,
}

use crate::List::{Cons, Nil};

// ------------------------------------------------------------------------ //

// Defining our own smart pointer
use std::ops::Deref;

struct MyBox<T>(T);

impl <T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

// ------------------------------------------------------------------------ //

fn main() {
	// Box<T> is a straight forward smart pointer that stores data on the heap.
	// It has no performance overhead other than storing data on heap than stack.
	// You'll use them in these situations:
	// - When you have a type whose size can’t be known at compile time and you 
	//   want to use a value of that type in a context that requires an exact size
	// - When you have a large amount of data and you want to transfer ownership 
	//   but ensure the data won’t be copied when you do so
	// - When you want to own a value and you care only that it’s a type that 
	//   implements a particular trait rather than being of a specific type
	
	// storing an i32 on the heap
	// b has a value of a Box that points to the value 5 allocated on the heap
	let b = Box::new(5);
	println!("b = {}", b);

	// RECURSIVE TYPES
	// A cons list in lisp: (1, (2, (3, Nil)))
	// in rust:
	// in list.rs:
	let list = Cons(1, 
				Box::new(Cons(2, 
				Box::new(Cons(3, 
				Box::new(Nil)
				)))));
	
	// REGULAR "POINTERS"
	let x = 5;
	let y = &x;
	assert_eq!(5, x);
	assert_eq!(5, *y);  // dereferencing y

	// THE BOX POINTER
	let x = 5;
	let y = Box::new(x);
	// The main difference from above is y is pointing to a COPIED value of x
	// in the heap rather than the value of x directly.
	assert_eq!(5, x);
	assert_eq!(5, *y); // dereferencing Box poointers is the same

	// Defining our own smart pointer
	let x = 5;
	let y = MyBox::new(x);
	assert_eq!(5, x);
	assert_eq!(5, *y);	

}
