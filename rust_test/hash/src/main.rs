use std::collections::HashMap;
fn main() {
	let mut a = HashMap::new();
	a.insert("pla".to_string(), 1);
	a.insert("plb".to_string(), 2);
	for i in &a {
		println!("{} {}", i.0, i.1);
	}
	println!("Hello, world!");
}
