#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

// use crate::{button::{button_configure_interrupt, button_init}, led::{led_init, led_off}};
use led::*;
use button::*;
use board::*;

mod startup_stm32f103; 
mod mcu;
mod board;
mod led;
mod button;
mod gpio;
mod reg;
mod proc;
mod exti;


#[unsafe(no_mangle)]
fn main() {

    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    loop {

    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        
    }
}

// button interrupt handler
unsafe fn _EXTI0_Handler() {
    _led_toggle(BLUE_LED_PORT, BLUE_LED_PIN);
}