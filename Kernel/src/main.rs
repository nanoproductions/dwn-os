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

use lib_gfx;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use dwn_os::memory::{self, BootInfoFrameAllocator};
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

	#[cfg(test)]
	test_main();
	// lib_gfx::painter::draw_square(10, 10, 50, 50);

	let mut executor: Executor = Executor::new();
	executor.spawn(Task::new(example_task()));
	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.run();

	// lib_gfx::create_GUI();

	// use lib_gfx::window;

	// let main_window = window::Window::new(10, 10, 300, 300);
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
