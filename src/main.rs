#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dwn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use dwn_os::println;

// VGA
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");

	dwn_os::init();

	// RECURSION _ OVERFLOW STACK
	fn stack_overflow() {
		stack_overflow();
	}

	// stack_overflow();

	// invoke breakpoint exception
	//  x86_64::instructions::interrupts::int3();

	// let mode = Graphics640x480x16::new();
	// mode.set_mode();
	// mode.clear_screen(Color16::Black);
	// mode.draw_line((80, 60), (80, 420), Color16::White);
	// mode.draw_line((80, 60), (540, 60), Color16::White);
	// mode.draw_line((80, 420), (540, 420), Color16::White);
	// mode.draw_line((540, 420), (540, 60), Color16::White);
	// mode.draw_line((80, 90), (540, 90), Color16::White);
	// for(offset, character) in "Text Editor".chars().enumerate() {
	// 	mode.draw_character(280 + offset * 8, 72, character, Color16::White);
	// }

	#[cfg(test)]
	test_main();

	println!("Did not crash!");
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
