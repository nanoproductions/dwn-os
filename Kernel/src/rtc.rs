use x86_64::instructions::port::{Port, PortWriteOnly};
use crate::serial_println;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Time {
    pub second: char,
    pub minute: char,
    pub hour: char,
    pub day: char,
    pub month: char,
    pub year: char
}

const CMOS_PORT: Port<u8> = Port::new(0x70);

impl Time {

    pub fn get_update_in_progress_flag() -> u8 {
        unsafe {CMOS_PORT.write(0x0A)};
        return (unsafe {CMOS_PORT.read()} & 0x80 );
    }

    pub fn get_rtc_register(reg: u8) {
       unsafe { CMOS_PORT.write(reg);}
    }

    // pub fn reading_something() {
    //     serial_println!("WE READING???");
    // }
}