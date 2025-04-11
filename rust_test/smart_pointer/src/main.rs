use std::rc::Rc;

#[derive(Debug)]
enum List {
	Cons(i32, Rc<List>),
	Nil
}

use List::{Cons, Nil};

fn main() {
	let a = Box::new(5);
	println!("{}", a);
	if *a == 5 {
		println!("same");
	}

	let _d = Rc::new(Cons(1, Rc::new(Cons(10, Rc::new(Nil)))));
	let _b = Cons(3, Rc::clone(&_d));
	let _c = Cons(4, Rc::clone(&_d));
	println!("{:?}", _d);
	
    println!("Hello, world!");
}
