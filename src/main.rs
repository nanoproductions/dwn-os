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

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

	use dwn_os::memory::translate_addr;
	use x86_64::VirtAddr;

	println!("#### DwnOS ####");

	println!("Hello World{}", "!");

	dwn_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	
	let addresses = [
		// vga buffer page
		0xb8000,
		// some code page
		0201008,
		// some stack page
		0x0100_0020_1a10,
		// virt add mapped to physical address 0
		boot_info.physical_memory_offset,
	];

	for &address in &addresses {
		// let virt = VirtAddr::new(address);
		// let phys = unsafe {translate_addr(virt, phys_mem_offset)};
		// println!("{:?} -> {:?}", virt, phys);
	}

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
	GUI::create_GUI();

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
