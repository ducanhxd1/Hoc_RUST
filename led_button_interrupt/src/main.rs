#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

// use crate::{button::{button_configure_interrupt, button_init}, led::{led_init, led_off}};
use board::*;
use button::*;
use led::*;

mod board;
mod button;
mod exti;
mod gpio;
mod led;
mod mcu;
mod proc;
mod reg;
mod startup_stm32f103;

#[unsafe(no_mangle)]
fn main() {
    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    led_on(BLUE_LED_PORT, BLUE_LED_PIN);

    led_init(GREEN_LED_PORT, GREEN_LED_PIN);

    button::button_init(
        USER_BTN_PORT,
        USER_BTN_PIN,
        Mode::Interrupt(Trigger::FallingEdge),
    );


    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

// button interrupt handler
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn EXTI0_Handler() {
    led_toggle(GREEN_LED_PORT, GREEN_LED_PIN);
    // clear the pending interrupt in the exti
    button::button_clear_interrupt(USER_BTN_PIN);
}
