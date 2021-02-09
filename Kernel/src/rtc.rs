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


const CENTURY_REGISTER = 0x00;
const CMOS_PORT: Port<u8> = Port::new(0x70);

impl Time {

    fn get_update_in_progress_flag() -> u8 {
        unsafe {CMOS_PORT.write(0x0A)};
        return (unsafe {CMOS_PORT.read()} & 0x80 );
    }

    fn get_rtc_register(reg: u8) {
       unsafe { CMOS_PORT.write(reg);}
    }

    fn read_rtc(&self) {
        let century: char;
        let last_second: char;
        let last_minute: char;
        let last_hour: char;
        let last_day: char;
        let last_month: char;
        let last_year: char;
        let last_century: char;
        let registerB: char;

        while (get_update_in_progress_flag());
        self.second = get_rtc_register(0x00);
        self.minute = get_RTC_register(0x02);
        self.hour = get_RTC_register(0x04);
        self.day = get_RTC_register(0x07);
        self.month = get_RTC_register(0x08);
        self.year = get_RTC_register(0x09);

        if(CENTURY_REGISTER !== 0) {
           century = get_rtc_register(CENTURY_REGISTER);
        }

        do {
            last_second = self.second;
            last_minute = self.minute;
            last_hour = self.hour;
            last_day = self.day;
            last_moth = self.month;
            last_century = century;

            while (get_update_in_progress_flag());
            self.second = get_rtc_register(0x00);
            self.minute = get_RTC_register(0x02);
            self.hour = get_RTC_register(0x04);
            self.day = get_RTC_register(0x07);
            self.month = get_RTC_register(0x08);
            self.year = get_RTC_register(0x09);
    
            if(CENTURY_REGISTER !== 0) {
               century = get_rtc_register(CENTURY_REGISTER);
            }
        } 
    }

    // pub fn reading_something() {
    //     serial_println!("WE READING???");
    // }
}