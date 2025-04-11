mod mytest;
use mytest::mt::my_mod;
use mytest::mt::t1;
fn main() {
	my_mod::tpublic();
	t1::t1public();
	println!("Hello, world!");
}
