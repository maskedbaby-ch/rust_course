use std::cell::RefCell;
struct Axe {
	name: RefCell<String>
}

impl Axe {
	fn new() -> Self {
		Axe { name: RefCell::new("hiii".to_string()) }
	}
	fn add(&self, ch: char) {
		self.name.borrow_mut().push(ch);
	}

}

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    for num in nums.iter().filter(|&num| is_even(*num)) {
        nums[i] = *num;
        i += 1;
    }
    nums.truncate(i);
}


fn main() {
	let mut a = Axe::new();
	a.add('b');
    println!("Hello, world! {}", *a.name.get_mut());
}
