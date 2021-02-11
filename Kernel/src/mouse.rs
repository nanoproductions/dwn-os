use ps2_mouse::{Mouse, MouseState};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::PortReadOnly;
use crate::{println, serial_println};

use lib_gfx;

lazy_static! {
    pub static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
}

pub fn init_mouse() {
    MOUSE.lock().init().unwrap();
    MOUSE.lock().set_on_complete(on_complete);
}

fn on_complete(mouse_state: MouseState) {
    println!("{:?}", mouse_state);
    serial_println!("{}, {}", mouse_state.get_x(), mouse_state.get_y());
}