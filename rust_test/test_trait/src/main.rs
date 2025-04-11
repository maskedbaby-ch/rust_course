use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

struct Apple
{
	name: String,
	weight: u32
}

impl Display for Apple
{
	fn fmt(&self, f:&mut Formatter)-> fmt::Result {
		write!(f, "name:{} weight:{}", self.name, self.weight)
	}
}

fn ftest<T>(t:T) -> i32
	where T: Display,
{
	println!("{}", t);
	2
}

fn main() {
	let a = 23;
	println!("hi {a}");
	ftest(1);
	ftest("11".to_string());
	ftest(Apple {name:"hh".to_string(), weight:11});
    println!("Hello, world!");
}
