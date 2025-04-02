#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod console;
mod  sbi;

fn clear_bss() {
	unsafe extern "C" {
		unsafe fn sbss();
		unsafe fn ebss();
	}
	(sbss as usize .. ebss as usize).for_each(|a| {
		unsafe {
			(a as *mut u8).write_volatile(0);
		}
	});
}

core::arch::global_asm!(include_str!("entry.asm"));
#[unsafe(no_mangle)]
extern "C" fn rust_main() {
	clear_bss();
	println!("Hello rust");
	sbi::shutdown();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
