/*
    Mouse Code from: https://github.com/shakram02/HamdOS/blob/master/src/mouse.rs
*/

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[cfg(test)]
use crate::serial_println;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MouseButtonState {
    left_button_clicked: bool,
    right_button_clicked: bool,
    middle_button_clicked: bool,
}

impl MouseButtonState {
    fn new() -> MouseButtonState {
        MouseButtonState {
            left_button_clicked: false,
            right_button_clicked: false,
            middle_button_clicked: false,
        }
    }

    fn parse_state(state: u8) -> MouseButtonState {
        MouseButtonState {
            left_button_clicked: (state & 0b00000001) != 0,
            right_button_clicked: (state & 0b00000010) != 0,
            middle_button_clicked: (state & 0b00000100) != 0,
        }
    }
}

/// Represents a mouse device. Please note
/// that this class doesn't accumulate state
/// i.e. it'll just send the deltas in mouse
/// positions, since the raw data needs filtering
/// and clipping to remain useful
/// https://wiki.osdev.org/PS/2_Mouse
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Mouse {
    current_x: isize,
    current_y: isize,
    current_z: isize,
    button_state: MouseButtonState,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            current_x: 0,
            current_y: 0,
            current_z: 0,
            button_state: MouseButtonState::new(),
        }
    }

    pub fn add_standard_packet(&mut self, packet: [u8; 4]) {
        let state = packet[0];

        // For some reason, QEMU sends back 2 mouse packets, the second one is always invalid
        // If you activate, disable then re-activate grabbing the value of the second packet
        // will change, but it'll remain an invalid mouse packet
        if state & 0x8 == 0 {
            return; // Always one bit isn't set. Invalid packet
        }

        self.button_state = MouseButtonState::parse_state(state);
        self.current_x = Mouse::get_signed_value(packet[1], state, 4);
        self.current_y = Mouse::get_signed_value(packet[2], state, 5);
        self.current_z = Mouse::get_z_value(packet[3]);
    }

    pub fn get_position(&self) -> Position {
        return Position {
            x: self.current_x,
            y: self.current_y,
            z: self.current_z,
        };
    }

    pub fn get_button_state(&self) -> MouseButtonState {
        self.button_state
    }

    fn get_signed_value(packet_value: u8, state: u8, bit_index: u8) -> isize {
        let is_negative = (state & (1 << bit_index)) != 0;
        let value = packet_value as u16;

        let ret: i16 = if is_negative {
            let mut v = (256 - value) as i16;
            v *= -1; // Add the -ve
            v
        } else {
            value as i16
        };

        return ret as isize;
    }

    fn get_z_value(packet_value: u8) -> isize {
        let z_axis = packet_value & 0x0F;
        if (z_axis & 0x8) != 0 {
            -1 * ((!z_axis + 1) & 0x0F) as isize
        } else {
            z_axis as isize
        }
    }
}

#[test_case]
fn test_buttons() {
    let s = 0b00000101;
    let mut m = Mouse::new();
    m.add_standard_packet([s, 0, 0, 0]);
    let mut bs = MouseButtonState::new();
    bs.middle_button_clicked = true;
    bs.left_button_clicked = true;
    assert_eq!(m.get_button_state(), bs);
    serial_println!("Mouse buttons...[ok]");
}