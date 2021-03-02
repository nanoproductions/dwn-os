#![no_std]
#![cfg_attr(test, no_main)]

use vga::writers::Graphics640x480x16;
use vga::{colors::Color16, writers::GraphicsWriter};

pub const MODE: Graphics640x480x16 = Graphics640x480x16::new();

pub mod painter;
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

pub struct Cursor {
    cursor_x: isize,
    cursor_y: isize,
}

impl Cursor {

	pub const fn init() -> Cursor {
		Cursor {
			cursor_x: 320,
			cursor_y: 240
		}
	}

    pub fn get_x(&self) -> isize {
        self.cursor_x
    }

    pub fn draw_initial_mouse(&self) {
        let x_center = 320;
        let y_center = 240;

        MODE.draw_line((x_center, y_center), (x_center + 4, y_center), Color16::Red);
        MODE.draw_line(
            (x_center + 4, y_center),
            (x_center, y_center - 4),
            Color16::Red,
        );
        MODE.draw_line((x_center, y_center - 4), (x_center, y_center), Color16::Red);
    }

    pub fn draw_mouse(&mut self, x: isize, y: isize) {

		self.cursor_x += x;
		self.cursor_y -= y;

        let x_main = self.cursor_x;
        let y_main = self.cursor_y;

        MODE.clear_screen(Color16::White);

        MODE.draw_line((x_main, y_main), (x_main + 4, y_main), Color16::Red);
        MODE.draw_line((x_main + 4, y_main), (x_main, y_main - 4), Color16::Red);
        MODE.draw_line((x_main, y_main - 4), (x_main, y_main), Color16::Red);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
