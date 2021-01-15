#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dwn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use dwn_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");

	dwn_os::init();

	// RECURSION _ OVERFLOW STACK
	fn stack_overflow() {
		stack_overflow();
	}

//	stack_overflow();

	// invoke breakpoint exception
	 x86_64::instructions::interrupts::int3();


	#[cfg(test)]
	test_main();

	println!("Did not crash!");
	loop {}
}

// Panic Function
#[cfg(not(test))] 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	dwn_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}
