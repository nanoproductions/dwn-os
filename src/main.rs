#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dwn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)] //new 

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

	use x86_64::registers::control::Cr3;

	let (level_4_page_table, _) = Cr3::read();
	println!("Level 4 page table as {:?}", level_4_page_table.start_address());

	// invoke breakpoint exception
	//  x86_64::instructions::interrupts::int3();

	#[cfg(test)]
	test_main();

	println!("Did not crash!");

	#[cfg(test)]
	test_main();

	println!("Did not crash!");


	// CREATE SHELL
	// Shell::create_shell();

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
