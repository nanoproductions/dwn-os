use crate::{print, println};
use pc_keyboard::DecodedKey;

pub fn create_shell() {
    print!("user$ ");

}

pub fn get_keyboard_input(key: DecodedKey) {
    // let mut keyboard_input = [];
    print!("I got {:?}", key);

    // println!("You typed {}", keyboard_input);
}