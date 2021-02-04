#![no_std]
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};
use dwn_os::serial;

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

	serial_println!("Hello from GUI!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
