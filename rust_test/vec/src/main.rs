struct Apple {
	name:String,
	fff: u32
}

fn main() {
	let mut a = vec![];
    println!("Hello, world!");
	let b = Apple {name:"d".to_string(), fff:12};
	a.push(b);
	for i in &a {
		println!("{} {}", i.name, i.fff);
	}
	let c = a.get(0);
	if let Some(v) = c {
		println!("{} {}", v.name, v.fff);
	}
	
}
