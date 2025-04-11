fn main() {
	let v = Some(11u8);
	match v {
        Some(a) => println!("{}", a),
        _ => println!("others"),
    }
    println!("Hello, world!");
}
