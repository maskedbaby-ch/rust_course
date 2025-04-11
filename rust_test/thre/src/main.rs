use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::ops::Add;
fn main() {
	let at = Arc::new(Mutex::new(1)); 
	let (tx, rx) = mpsc::channel();
	let d = Arc::clone(&at);
	let d1 = Arc::clone(&at);
    println!("Hello, world!");
	let a = thread::spawn(move || {
		for i in 0..5 {
			println!("tx {}", i);
			let mut data = d.lock().unwrap();
			*data += 1;
			tx.send("hi").unwrap();
		}
	});
	let b = thread::spawn(move || {
		for i in 0..10 {
			println!("rx {}", i);
			if let Ok(v) = rx.try_recv() {
				println!("recv:{}", v);
				let mut data = d1.lock().unwrap();
				*data += 2;
			}
		}
	});

	a.join().unwrap();
	b.join().unwrap();
	let d = at.lock().unwrap();
	println!("{:?}", d);
}
