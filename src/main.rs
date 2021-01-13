#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

// Panic Function
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");

	#[cfg(test)]
	test_main();

	loop {}
}

// Testing
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
}

fn trivial_assertion() {
	print!("trivial assertion... ");
	assert_eq!(1, 1);
	println!("[ok]");
}
