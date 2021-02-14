#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dwn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)] //new 

use core::panic::PanicInfo;
use dwn_os::{println, print, serial_println};
use dwn_os::interrupts;

use lib_gfx;

use x86_64::{structures::paging::Page, VirtAddr};

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

	use dwn_os::memory;
	use x86_64::VirtAddr;
	use x86_64::structures::paging::MapperAllSizes;
	use dwn_os::memory::BootInfoFrameAllocator;

	println!("#### DwnOS ####");

	println!("Hello World{}", "!");

	dwn_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper  = unsafe { memory::init(phys_mem_offset) };

	let mut frame_allocator = memory::EmptyFrameAllocator;

	// map an unused page
	let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
	memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

	// write the string "New!" to the screen
	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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
		let virt = VirtAddr::new(address);
		let phys = mapper.translate_addr(virt);
		println!("{:?} -> {:?}", virt, phys);
	}

	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	#[cfg(test)]
	test_main();

	println!("Did not crash!");

	#[cfg(test)]
	test_main();

	println!("Did not crash!");


	// CREATE SHELL
	// Shell::create_shell();

	// lib_gfx::create_GUI();

	// use lib_gfx::window;

	// let main_window = window::Window::new(10, 10, 300, 300);

	// serial_println!("Creating window...");

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
