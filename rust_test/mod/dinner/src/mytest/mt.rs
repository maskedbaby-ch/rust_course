pub mod t1 {
	pub fn t1public() {
		println!("t1private");
	}
}

pub mod my_mod{
	fn tprivate() {
		println!("private");
	}
	pub fn tpublic() {
		println!("public");
		tprivate();
	}
}
