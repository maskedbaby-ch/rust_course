use std::io::stdin;
mod modtest;

use modtest::{math::my_hello::hello, my_module::Hence};
use modtest::my_module::{self, button, ret_func, Anya};

fn main() {
    let a = vec![1, 2, 3];
    let b: Vec<_> = a.iter().map(|x| x * x).collect();
    println!("{:?}", b);
    let mut a: Vec<&str> = vec!["aa", "b", "dd"];
    a.push("cc");
    let b: Vec<_> = a.iter().map(|x| x.to_uppercase()).collect();
    println!("{:?}", b);
	let a = my_module::Afunc {name:"hh".to_string(), function:12};
	a.afunc();
	a.draw();
	button(&a);
	hello();

	let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
        
    let i: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
             println!("wrong");
             return;
        }
    };
	ret_func(i.try_into().unwrap()).afunc();
}

