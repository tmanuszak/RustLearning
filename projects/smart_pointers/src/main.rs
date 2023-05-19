#![allow(unused)]

// REASONS TO CHOOSE THE VARIOUS SMART POINTERS
// - Rc<T> enables multiple owners of the same data; 
//   Box<T> and RefCell<T> have single owners.
// - Box<T> allows immutable or mutable borrows checked at compile time; 
//   Rc<T> allows only immutable borrows checked at compile time; 
//   RefCell<T> allows immutable or mutable borrows checked at runtime.
// - Because RefCell<T> allows mutable borrows checked at runtime, you can 
//   mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

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

// Custom smart pointer for drop trait demonstrations
struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data '{}'!", self.data);
	}
}

use std::mem::drop;

// ------------------------------------------------------------------------ //

// FOR Rc POINTER DEMONSTRATION
enum ListRc {
	ConsRc(i32, Rc<ListRc>),
	NilRc,
}

use crate::ListRc::{ConsRc, NilRc};
use std::rc::Rc;

// ------------------------------------------------------------------------ //

// FOR RefCell POINTER DEMONSTRATION
pub trait Messenger {
	fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
	messenger: &'a T,
	value: usize,
	max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
	T: Messenger,
{
	pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
		LimitTracker {
			messenger,
			value: 0,
			max,
		}
	}
	
	pub fn set_value(&mut self, value: usize) {
		self.value = value;

		let percentage_of_max = self.value as f64 / self.max as f64;

		if percentage_of_max >= 1.0 {
			self.messenger.send("Error: You are over your quota!");
		} else if percentage_of_max >= 0.9 {
			self.messenger
				.send("Urgent warning: You've used up over 90% of your quota!");
		} else if percentage_of_max >= 0.75 {
			self.messenger
				.send("Warning: You've used up over 75% of your quota!");
		}
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
	
	// DROP TRAIT DEMONSTRATIONS
	{
		let c = CustomSmartPointer {
			data: String::from("my stuff"),
		};
		let d = CustomSmartPointer {
			data: String::from("other stuff"),
		};
		let e = CustomSmartPointer {
			data: String::from("some data"),
		};
		println!("CustomSmartPointers created.");
		drop(e);  // Not allowed to call c.drop(), use this instead
		println!("CustomSmartPointer dropped before the end of scope.");
	}

	// REFERENCE COUNTED POINTER
	// The Rc<T> smart pointer type allowes a single value to have multiple 
	// owners. It keep track of the number of owners and free the data when
	// there are 0 owners.
	// NOTE: it is only for use in single threaded scenarios.
	// Creating two lists that take ownership of another list:
	let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
	println!("Count after creating a = {}", Rc::strong_count(&a));
	let b = ConsRc(3, Rc::clone(&a));
	println!("Count after creating b = {}", Rc::strong_count(&a));
	{
		let c = ConsRc(4, Rc::clone(&a));
		println!("Count after creating c = {}", Rc::strong_count(&a));
	}
	println!("Count after c goes out of scope = {}", Rc::strong_count(&a));

}
