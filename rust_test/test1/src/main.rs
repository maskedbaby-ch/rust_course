use std::io;
fn main()
{
	println!("input");
	let mut data = String::new();
	io::stdin().read_line(&mut data).expect("wrong read data");
	let num: u32 = "42".parse().expect("not number");
	println!("{}", num);
	danger();
}

fn danger()->String 
{
	String::from("hello")
}
