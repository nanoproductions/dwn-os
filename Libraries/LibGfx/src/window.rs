use vga::writers::GraphicsWriter;
use crate::MODE;
use vga::colors::Color16;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Window {
    position_x: isize,
    position_y: isize,
    height: isize,
    width: isize,
}

impl Window {
    pub fn new(x: isize, y: isize, height: isize, width: isize) -> Window {
        
        let window = Window {
            position_x: x,
            position_y: y,
            height: height,
            width: width
        };

        window.draw_window();

        return window;
    }

    fn draw_window(self) {
        MODE.draw_line((self.position_x, self.position_y), (self.position_x + self.width, self.position_y), Color16::Magenta);
        MODE.draw_line((self.position_x + self.width, self.position_y), (self.position_x + self.width, self.position_y + self.height), Color16::Magenta);
        MODE.draw_line((self.position_x + self.width, self.position_y + self.height), (self.position_x, self.position_y + self.height), Color16::Magenta);
        MODE.draw_line((self.position_x, self.position_y + self.height), (self.position_x, self.position_y), Color16::Magenta);

        MODE.draw_line((self.position_x + 1, self.position_y + 1), (self.position_x + 1 + self.width, self.position_y + 1), Color16::Magenta);
        MODE.draw_line((self.position_x + 1 + self.width, self.position_y + 1), (self.position_x + 1 + self.width, self.position_y + 1 + self.height), Color16::Magenta);
        MODE.draw_line((self.position_x + 1 + self.width, self.position_y + 1 + self.height), (self.position_x + 1, self.position_y + 1 + self.height), Color16::Magenta);
        MODE.draw_line((self.position_x + 1, self.position_y + 1 + self.height), (self.position_x + 1, self.position_y + 1), Color16::Magenta);
    }
}
