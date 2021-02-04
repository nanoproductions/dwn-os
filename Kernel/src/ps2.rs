/*
    PS2 Code from: https://github.com/shakram02/HamdOS/blob/master/src/mouse.rs
*/

use spin::Mutex;
use x86_64::instructions::port::Port;

use lazy_static::lazy_static;

use crate::println;

const CMD_READ_CONFIG_BYTE: u8 = 0x20;
const CMD_WRITE_CONFIG_BYTE: u8 = 0x60;
const CMD_ENABLE_FIRST_PORT: u8 = 0xAE;
const CMD_DISABLE_FIRST_PORT: u8 = 0xAD;
const CMD_ENABLE_SECOND_PORT: u8 = 0xA8;
const CMD_DISABLE_SECOND_PORT: u8 = 0xA7;
const CMD_TEST_PS2_CONTROLLER: u8 = 0xAA;
const CMD_TEST_PS2_FIRST_PORT: u8 = 0xAB;
const CMD_TEST_PS2_SECOND_PORT: u8 = 0xA9;
#[allow(dead_code)]
const DEVICE_CMD_DISABLE_SCANNING: u8 = 0xF5;
const DEVICE_MOUSE_ENABLE_DATA_REPORTING: u8 = 0xF4;
const DEVICE_MOUSE_SET_SAMPLE_RATE: u8 = 0xF3;
const DEVICE_MOUSE_RESET: u8 = 0xFF;
// https://wiki.osdev.org/PS/2_Mouse
const DEVICE_CMD_IDENTIFY: u8 = 0xF2;
const CMD_SEND_TO_SECOND_PORT_INPUT_BUFFER: u8 = 0xD4;
// Sends an input to second devices
const PORT_NO_COMMAND: u16 = 0x64;
const PORT_NO_DATA: u16 = 0x60;
const REPLY_CONTROLLER_TEST_PASS: u8 = 0x55;
const REPLY_DEVICE_ACK: u8 = 0xFA;
const REPLY_DEVICE_RESEND: u8 = 0xFE;

lazy_static! {
    static ref PS2: Mutex<Ps2Controller> = Mutex::new(Ps2Controller::new());
}

pub fn init() {
    // PS/2 controller initialization https://wiki.osdev.org/%228042%22_PS/2_Controller
    let controller: &mut Ps2Controller = &mut PS2.lock();
    controller.disable_first_device(); // Turns out to be the keyboard
    controller.disable_second_device();
    controller.flush_output_buffer();
    controller.disable_irqs_and_translation();
    controller.perform_self_test();
    controller.perform_interface_tests();
    controller.enable_devices_and_translation();

    println!(
        "Found PS/2 device [0]: {:#?}",
        controller.identify_port_device(Ps2Port::One)
    );
    if controller.is_dual_channel {
        println!(
            "Found PS/2 device [1]: {:#?}",
            controller.identify_port_device(Ps2Port::Two)
        );
        //        controller.write_to_port_two_device()
        controller.write_to_port_two_device(DEVICE_MOUSE_RESET);

        // Enter scrolling wheel mode
        for _ in 0..2 {
            controller.write_to_port_two_device(DEVICE_MOUSE_SET_SAMPLE_RATE);
            controller.write_to_port_two_device(200);
        }
        controller.write_to_port_two_device(DEVICE_MOUSE_SET_SAMPLE_RATE);
        controller.write_to_port_two_device(80);

        controller.write_to_port_two_device(DEVICE_MOUSE_ENABLE_DATA_REPORTING);
        controller.read_ack();
    }

    controller.enable_irqs();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Ps2Port {
    One,
    Two,
}

#[derive(Debug, Copy, Clone)]
enum AckType {
    Resend,
    Ok,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum DeviceType {
    Ps2Mouse = 0x00,
    MouseScrollWheel = 0x03,
    FiveButtonMouse = 0x04,
    MF2KeyboardWithTranslation = 0x41,
    MF2KeyboardWithTranslationDup = 0xC1,
    MFKeyboard = 0x83,
}

#[derive(Debug)]
struct Ps2Controller {
    command_port: Port<u8>,
    data_port: Port<u8>,
    is_dual_channel: bool,
}

impl Ps2Controller {
    pub const fn new() -> Ps2Controller {
        Ps2Controller {
            command_port: Port::new(PORT_NO_COMMAND),
            data_port: Port::new(PORT_NO_DATA),
            is_dual_channel: false,
        }
    }

    fn disable_first_device(&mut self) {
        self.write_command(CMD_DISABLE_FIRST_PORT);
    }

    fn disable_second_device(&mut self) {
        self.write_command(CMD_DISABLE_SECOND_PORT);
    }

    /// Disables interrupt for all devices and clears translation
    /// Represents step #5 in PS/2 controller initialization
    /// Also determines if the PS/2 controller is dual channel
    /// by setting is_dual_channel
    fn disable_irqs_and_translation(&mut self) {
        let mut config = self.read_configuration_byte();
        // Disable first and second device interrupts and disable translation [0,1,6]
        config = config & 0b10111100;

        // Check for second port enabled
        // Second PS/2 port clock (1 = disabled, 0 = enabled, only if 2 PS/2 ports supported)
        if (config & 0x10) != 0 {
            self.is_dual_channel = true;
        }

        self.write_configuration_byte(config);
    }

    /// Enable any PS/2 port that exists and works.
    /// If you're using IRQs (recommended), also enable interrupts
    /// for any PS/2 ports in the Controller Configuration Byte.
    fn enable_devices_and_translation(&mut self) {
        // If any device failed, we would've paniced
        self.write_command(CMD_ENABLE_FIRST_PORT);
        self.write_command(CMD_ENABLE_SECOND_PORT);
        let mut config = self.read_configuration_byte();
        config |= 0b01000000;
        self.write_command(CMD_WRITE_CONFIG_BYTE);
        self.write_data(config);
    }

    fn enable_irqs(&mut self) {
        let mut config = self.read_configuration_byte();
        // IRQs are on bits 0,1 and enable translation 6
        config |= if self.is_dual_channel {
            0b00000011 // Enable First and Second ports
        } else {
            0b00000001 // Enable First port
        };

        self.write_command(CMD_WRITE_CONFIG_BYTE);
        self.write_data(config);
    }

    /// Performs controller self test and panics
    /// if the test failed
    fn perform_self_test(&mut self) {
        let config = self.read_configuration_byte();

        self.write_command(CMD_TEST_PS2_CONTROLLER);
        let first_port_status = self.read_data();
        if first_port_status != REPLY_CONTROLLER_TEST_PASS {
            panic!("PS/2 controller is malfunctioning")
        }

        // For the devices that may reset
        self.write_configuration_byte(config)
    }

    /// Performs device test and panics
    /// if the test failed
    fn perform_interface_tests(&mut self) {
        self.write_command(CMD_TEST_PS2_FIRST_PORT);
        if self.read_data() != 0 {
            panic!("First PS/2 device failed")
        }

        if !self.is_dual_channel {
            return;
        }

        self.write_command(CMD_TEST_PS2_SECOND_PORT);
        let response = self.read_data();
        if response != 0 {
            panic!("Second PS/2 device failed")
        }
    }

    fn write_to_port_one_device(&mut self, cmd: u8) {
        self.write_data(cmd);
    }

    fn write_to_port_two_device(&mut self, cmd: u8) {
        self.write_command(CMD_SEND_TO_SECOND_PORT_INPUT_BUFFER);
        self.write_data(cmd)
    }

    fn identify_port_device(&mut self, port: Ps2Port) -> DeviceType {
        fn send_func(controller: &mut Ps2Controller, port: Ps2Port, cmd: u8) {
            if port == Ps2Port::Two {
                controller.write_to_port_two_device(cmd)
            } else {
                controller.write_to_port_one_device(cmd)
            };
        }

        // Send ->
        // ACK <-
        // Response <-
        send_func(self, port, DEVICE_CMD_IDENTIFY);
        loop {
            match self.read_ack() {
                AckType::Resend => {
                    send_func(self, port, DEVICE_CMD_IDENTIFY);
                }
                AckType::Ok => break,
            }
        }

        // Now the type is ready to be read
        let mut response = self.read_data();
        if response == (0xAB as u8) {
            // 2-byte response
            response = self.read_data();
        }

        match response {
            0x00 => DeviceType::Ps2Mouse,
            0x03 => DeviceType::MouseScrollWheel,
            0x04 => DeviceType::FiveButtonMouse,
            0x41 => DeviceType::MF2KeyboardWithTranslation,
            0xC1 => DeviceType::MF2KeyboardWithTranslationDup,
            0x83 => DeviceType::MFKeyboard,
            _ => panic!("Undefined device type: {}", response),
        }
    }

    fn read_ack(&mut self) -> AckType {
        let response = self.read_data();
        if response == REPLY_DEVICE_RESEND {
            AckType::Resend
        } else if response == REPLY_DEVICE_ACK {
            AckType::Ok
        } else {
            panic!("Device didn't send an ACK [{:X}]", response)
        }
    }

    fn read_configuration_byte(&mut self) -> u8 {
        self.write_command(CMD_READ_CONFIG_BYTE);
        self.read_data()
    }

    fn write_configuration_byte(&mut self, config: u8) {
        self.write_command(CMD_WRITE_CONFIG_BYTE);
        self.write_data(config);
    }

    fn write_command(&mut self, cmd: u8) {
        self.wait_till_input_buffer_empty();
        self.write_command_direct(cmd)
    }

    fn wait_till_out_buffer_full(&mut self) {
        while self.check_output_buffer_full() {}
    }

    fn wait_till_input_buffer_empty(&mut self) {
        while self.check_input_buffer_empty() {}
    }

    fn check_output_buffer_full(&mut self) -> bool {
        (self.read_status_register_direct() & 0x1) == 0
    }

    fn check_input_buffer_empty(&mut self) -> bool {
        (self.read_status_register_direct() & 0x1 << 1) != 0
    }

    fn read_status_register_direct(&mut self) -> u8 {
        unsafe { self.command_port.read() }
    }

    fn write_command_direct(&mut self, cmd: u8) {
        unsafe {
            self.command_port.write(cmd);
        }
    }

    fn write_data(&mut self, data: u8) {
        self.wait_till_input_buffer_empty();
        unsafe { self.data_port.write(data) }
    }

    fn read_data(&mut self) -> u8 {
        self.wait_till_out_buffer_full();
        self.read_data_direct()
    }

    fn read_data_direct(&mut self) -> u8 {
        unsafe { self.data_port.read() }
    }

    /// Forces flush of controller's output buffer
    fn flush_output_buffer(&mut self) {
        unsafe {
            while (self.command_port.read() & 0x1) != 0 {
                self.data_port.read();
            }
        }
    }
}
