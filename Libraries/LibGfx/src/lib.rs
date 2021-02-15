#![no_std]
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

pub const MODE: Graphics640x480x16 = Graphics640x480x16::new();
// static mut PREVIOUS_MOUSE: [usize; 2] = [0, 0];

pub mod window;

pub fn create_gui() {
	MODE.set_mode();
	MODE.clear_screen(Color16::White);

	// for x in 0..20 {
	// 	MODE.draw_line((0, 0 + x), (640, 0 + x), Color16::Black);
	// }

	// for(offset, character) in "9:41".chars().enumerate() {
	// 	MODE.draw_character(288 + offset * 8, 5, character, Color16::White);
	// }

	// MODE.draw_line((1, 10), (5, 10), Color16::Magenta);
	// MODE.draw_line((80, 60), (80, 420), Color16::Black);
	// MODE.draw_line((80, 60), (540, 60), Color16::Black);
	// MODE.draw_line((80, 420), (540, 420), Color16::Black);
	// MODE.draw_line((540, 420), (540, 60), Color16::Black);
	// MODE.draw_line((80, 90), (540, 90), Color16::Black);

	// mode.set_pixel(usize(Mouse.current_x), Mouse::current_y, Color16::Red);
	// for(offset, character) in "Text Editor".chars().enumerate() {
	// 	MODE.draw_character(280 + offset * 8, 72, character, Color16::Black);
	// }

	// serial_println!("Hello from GUI!");
}

pub fn draw_mouse() {
	MODE.clear_screen(Color16::Red);
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
