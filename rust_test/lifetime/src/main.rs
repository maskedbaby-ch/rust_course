fn longest<'m>(a:&'m String, b:&'m String) -> &'m String
{
	if a.len() > b.len() {
		a
	}else{
		b
	}
}

fn main() {
	let s1 = String::from("hhh");
	let s2 = String::from("llkkl");
	let a = longest(&s1,&s2);
	println!("{}", a);
    println!("Hello, world!");
}
