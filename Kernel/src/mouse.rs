use ps2_mouse::{Mouse, MouseState};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::PortReadOnly;
use crate::println;

lazy_static! {
    pub static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
}

pub fn init_mouse() {
    MOUSE.lock().init().unwrap();
    MOUSE.lock().set_on_complete(on_complete);
}

fn on_complete(mouse_state: MouseState) {
    println!("{:?}", mouse_state);
}