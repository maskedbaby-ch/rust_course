fn main() {
	let mut a = 32; 
	unsafe {
		let b = &mut a as *mut i32; //可变指针
		*b = 33;
		println!("{}", *b);
	}
	let mut b = 11;
	unsafe {
		let x = &b as *const i32; //不可变
		println!("{}", *x);
	}

    // You can optionally experiment here.
}

