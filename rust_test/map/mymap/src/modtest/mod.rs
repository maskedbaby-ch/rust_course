pub mod math;

pub mod my_module {
	pub trait Hence {
		fn afunc(&self) {}
	}
	pub trait Anya: Hence {
		fn draw(&self) {}
	}
	pub struct Afunc {
		pub name: String,
		pub function: u32,
	}
	pub struct Bfunc {
		pub name: u32,
		pub function: u32,
	}
	
	impl Hence for Afunc {
		fn afunc(&self) {
			println!("{} {}", self.name, self.function);
		}
	}

	impl Anya for Afunc {
		fn draw(&self) {
			println!("draw Anya");
		}
	}

	impl Hence for Bfunc {
		fn afunc(&self) {
			println!("{} {}", self.name, self.function);
		}
	}

	pub fn button<T:Hence>(a:&T) {
		a.afunc();
	}

	pub fn ret_func(a: u32) -> Box<dyn Hence>{
		if a == 1 {
			Box::new(Afunc {name:"ret".to_string(), function:12})
		} else {
			Box::new(Bfunc {name:11, function:12})
		}
	}
}


// let h:Vec<Box<dyn Hence>>=vec![Box::new(Afunc {name:"hh".to_string(), function:12}),
// 							Box::new(Bfunc {name:156, function:12})]; //dyn dispatch
// for  i in h {
// 	i.afunc();
// }




