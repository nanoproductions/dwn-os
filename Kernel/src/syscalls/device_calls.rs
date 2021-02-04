pub mod sys_keyboard {
    let keyboard_input;


    fn send_keyboard_input(input) {
        keyboard_input = input;
    }

    fn get_keyboard_input() {
        keyboard_input
    }

}