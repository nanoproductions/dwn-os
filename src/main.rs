#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dwn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use dwn_os::Shell;
use dwn_os::{println, print, serial_println};
use dwn_os::interrupts;
use dwn_os::GUI;

// VGA
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};


#[no_mangle]
pub extern "C" fn _start() -> ! {

	println!("###########");
	println!("#### DwnOS ####");
	println!("###########");

	println!("Hello World{}", "!");

	dwn_os::init();

	// RECURSION _ OVERFLOW STACK
	fn stack_overflow() {
		stack_overflow();
	}

	// stack_overflow();

	// invoke breakpoint exception
	//  x86_64::instructions::interrupts::int3();

	#[cfg(test)]
	test_main();

	println!("Did not crash!");
	println!("Switching to Graphics Mode...");

	// loop { for _ in 0..1000000 {}; break; };

	#[cfg(test)]
	test_main();

	println!("Did not crash!");


	// CREATE SHELL
	Shell::create_shell();

	// Create GUI
	// GUI::create_GUI();

	dwn_os::hlt_loop();
}

// Panic Function
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	dwn_os::hlt_loop();
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
