use crate::print;
use alloc::string::String;

struct Shell {
    current_line: String
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            current_line: String::from(""),
        }
    }

    pub fn init(&self) {
        print!("user$ ");
    }
}