
enum ButtonStatus {
    Pressed,
    Released
}

pub fn button_init(_pin: i32) {

}

pub fn button_configure_interrupt(_pin: i32) {

}

pub fn button_read_status(_pin: i32) -> ButtonStatus {
    ButtonStatus::Pressed
}