// File này chứa tất cả các chi tiết về bo mạch như có bao nhiêu LED có sẵn chân, các button sẵn có.

use crate::mcu::*;

pub const BLUE_LED_PIN: u32 = GPIO_PIN_13;
pub const BLUE_LED_PORT: u32 = GPIOC_BASE;

pub const GREEN_LED_PIN: u32 = GPIO_PIN_0;
pub const GREEN_LED_PORT: u32 = GPIOB_BASE;

pub const USER_BTN_PORT: u32 = GPIOA_BASE;
pub const USER_BTN_PIN: u32 = GPIO_PIN_0;
