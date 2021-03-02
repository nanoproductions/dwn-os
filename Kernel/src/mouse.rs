use ps2_mouse::{Mouse, MouseState};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::{println, serial_println};

use lib_gfx;
use lib_gfx::Cursor;

lazy_static! {
    pub static ref CURSOR: Mutex<Cursor> = Mutex::new(Cursor::init());
    pub static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
}

pub fn init_mouse() {
    MOUSE.lock().init().unwrap();
    MOUSE.lock().set_on_complete(on_complete);
}

fn on_complete(mouse_state: MouseState) {
    println!("{:?}", mouse_state);
    // serial_println!("{}, {}", mouse_state.get_x(), mouse_state.get_y());
    if mouse_state.x_moved() && mouse_state.y_moved() {

        // CURSOR.cursor_x = CURSOR.cursor_x + mouse_state.get_x() as isize;
        // CURSOR.lock().cursor_y = CURSOR.lock().cursor_y + mouse_state.get_y() as isize;

        serial_println!("{}", CURSOR.lock().cursor_y);
        CURSOR.lock().draw_mouse(mouse_state.get_x() as isize, mouse_state.get_y() as isize);
        // serial_println!("x mouse{:?}", mouse_state.get_x());
        // serial_println!("y mouse{:?}", mouse_state.get_y());
        // serial_println!("code says.... {}", CURSOR.cursor_x);
    }
}