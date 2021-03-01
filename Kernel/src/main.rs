#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dwn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)] //new

use core::panic::PanicInfo;
use dwn_os::println;

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use alloc::string::String;
use dwn_os::task::{Task, simple_executor::SimpleExecutor};
use dwn_os::task::executor::Executor;
use dwn_os::task::keyboard;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use dwn_os::memory::{self, BootInfoFrameAllocator};
	use x86_64::structures::paging::MapperAllSizes;
	use x86_64::VirtAddr;
	use dwn_os::allocator;

	println!("#### DwnOS ####");

	println!("Hello World{}", "!");

	dwn_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };

	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

	// Allocate number on the heap
	let heap_value = Box::new(41);	
	println!("heap_value at {:p}", heap_value);

	// create dynamically sized vector
	let mut vec = Vec::new();
	for i in 0..500 {
		vec.push(i);
	}
	println!("vec at {:p}", vec.as_slice());

	// create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

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

	let mut executor = Executor::new();
	executor.spawn(Task::new(example_task()));
	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.run();

	#[cfg(test)]
	test_main();

	println!("Did not crash!");

	#[cfg(test)]
	test_main();

	println!("Did not crash!");

	// lib_gfx::create_GUI();

	// use lib_gfx::window;

	// let main_window = window::Window::new(10, 10, 300, 300);

	// serial_println!("Creating window...");

	dwn_os::hlt_loop();
}

async fn async_number() -> u32 {
	42
}

async fn example_task() {
	let number = async_number().await;
	println!("async number: {}", number);
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
