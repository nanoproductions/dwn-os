#![no_std]

use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

use crate::mouse::Mouse;
use crate::serial_println;

use core::convert::TryInto;

pub const MODE: Graphics640x480x16 = Graphics640x480x16::new();

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
	serial_println!("Mouse: ({}, {})", x, y);

	x = x * -1;
	y = y * -1;

	let x_pos = x as usize;
	let y_pos = y as usize;
	MODE.set_pixel(x_pos, y_pos, Color16::Red);
}