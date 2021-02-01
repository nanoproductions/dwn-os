#![no_std]

use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

use crate::mouse::Mouse;
use crate::serial_println;

use core::convert::TryInto;

pub const MODE: Graphics640x480x16 = Graphics640x480x16::new();
static mut PREVIOUS_MOUSE: [usize; 2] = [0, 0];

pub fn create_GUI() {
	MODE.set_mode();
	MODE.clear_screen(Color16::White);
	MODE.draw_line((80, 60), (80, 420), Color16::Black);
	MODE.draw_line((80, 60), (540, 60), Color16::Black);
	MODE.draw_line((80, 420), (540, 420), Color16::Black);
	MODE.draw_line((540, 420), (540, 60), Color16::Black);
	MODE.draw_line((80, 90), (540, 90), Color16::Black);

	// mode.set_pixel(usize(Mouse.current_x), Mouse::current_y, Color16::Red);
	for(offset, character) in "Text Editor".chars().enumerate() {
		MODE.draw_character(280 + offset * 8, 72, character, Color16::Black);
	}
}

pub fn draw_mouse(mut x: isize, mut y: isize) {
	// MODE.set_pixel(unsafe{PREVIOUS_MOUSE[0]}, unsafe{PREVIOUS_MOUSE[1]}, Color16::White);
	// MODE.set_pixel(unsafe{PREVIOUS_MOUSE[0]} + 1, unsafe{PREVIOUS_MOUSE[1]} + 1, Color16::White);
	// MODE.set_pixel(unsafe{PREVIOUS_MOUSE[0]} - 1, unsafe{PREVIOUS_MOUSE[1]} - 1, Color16::White);
	// MODE.set_pixel(unsafe{PREVIOUS_MOUSE[0]}, unsafe{PREVIOUS_MOUSE[1]} + 2, Color16::White);
	// MODE.set_pixel(unsafe{PREVIOUS_MOUSE[0]}, unsafe{PREVIOUS_MOUSE[1]} + 3, Color16::White);
	
	/*
	MODE.set_pixel(x_pos - 1, y_pos - 1, Color16::Red);
	MODE.set_pixel(x_pos, y_pos + 2, Color16::Red);
	MODE.set_pixel(x_pos, y_pos + 3, Color16::Red);
	*/

	serial_println!("Mouse: ({}, {})", x, y);

	x = x * -1;
	y = y * -1;

	let x_pos = x as usize;
	let y_pos = y as usize;

	// unsafe {PREVIOUS_MOUSE = [x_pos, y_pos];}
	// serial_println!("Previous mouse is: {:?}", unsafe {PREVIOUS_MOUSE});
	// MODE.set_pixel(x_pos, y_pos, Color16::Red);
	// MODE.set_pixel(x_pos + 1, y_pos + 1, Color16::Red);
	// MODE.set_pixel(x_pos - 1, y_pos - 1, Color16::Red);
	// MODE.set_pixel(x_pos, y_pos + 2, Color16::Red);
	// MODE.set_pixel(x_pos, y_pos + 3, Color16::Red);
}