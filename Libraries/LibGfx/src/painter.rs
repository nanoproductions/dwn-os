use vga::writers::GraphicsWriter;
use crate::MODE;
use vga::colors::Color16;

pub fn draw_square(width: isize, height: isize, position_x: isize, position_y: isize) {
    MODE.draw_line((position_x, position_y), (position_x + width, position_y), Color16::Magenta);

    MODE.draw_line((position_x + width, position_y), (position_x + width, position_y + height), Color16::Magenta);

    MODE.draw_line((position_x + width, position_y + height), (position_x, position_y + height), Color16::Magenta);
    
    MODE.draw_line((position_x, position_y + height), (position_x, position_y), Color16::Magenta);
}