#![no_std]
#![no_main]

use core::panic::PanicInfo;
use dwn_os::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    dwn_os::gdt::init();
    init_test_idt();

    // trigger stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dwn_os::test_panic_handler(info);
}

// CONTINUE
// THE TEST IDT