//! # Module Title
//!
//! This module provides [a brief overview of what the module does].
//!
//! It contains functions, structs, and enums to [explain key features or functionalities].
//!
//! ---
//!
//! The following items are included in this module:
//! - `EnumName`: Describes the different states or categories.
//! - `StructName`: Contains relevant data and methods.
//! - Functions like `function_name`: Describe what each function does.
//!
use crate::mcu::*;
use crate::reg::*;

pub fn enable_gpio_clock(port: u32) {
    // 0x18 offset
    let rcc_apb2enr_addr = (RCC_BASE + 0x18) as *mut u32;

    match port {
        GPIOA_BASE => {
            //enable the 2th bit  of rcc_apb2enr_addr
            reg_set_bit(rcc_apb2enr_addr, 2, true);
        }

        GPIOB_BASE => {
            reg_set_bit(rcc_apb2enr_addr, 3, true);
        }

        GPIOC_BASE => {
            reg_set_bit(rcc_apb2enr_addr, 4, true);
        }

        GPIOD_BASE => {
            reg_set_bit(rcc_apb2enr_addr, 5, true);
        }

        GPIOE_BASE => {
            reg_set_bit(rcc_apb2enr_addr, 6, true);
        }

        AFIO_BASE => {
            reg_set_bit(rcc_apb2enr_addr, 0, true);
        }
        _ => {} // //catch all pattern, do nothing for values other than GPIOA_BASE
    }
}

pub enum GpioMode {
    InputAnalog,
    InputFloating,
    InputPullUpDown,
    OutputPushPull(u8), // tốc độ: 10, 2, 50 MHz
    OutputOpenDrain(u8),
    AltPushPull(u8),
    AltOpenDrain(u8),
}

pub fn set_gpio_mode(port: u32, pin: u32, mode: GpioMode) {
    assert!(pin < 16, "Pin must be between 0 and 15");

    // Chọn CRL hoặc CRH
    let (reg_offset, bit_position) = if pin < 8 {
        (0x00, pin * 4)
    } else {
        (0x04, (pin - 8) * 4)
    };

    let gpio_mode_reg_addr = (port + reg_offset) as *mut u32;

    // MODE & CNF
    let (cnf_bits, mode_bits) = match mode {
        GpioMode::InputAnalog => (0b00, 0b00),
        GpioMode::InputFloating => (0b01, 0b00),
        GpioMode::InputPullUpDown => (0b10, 0b00),
        GpioMode::OutputPushPull(speed) => (0b00, mode_speed_bits(speed)),
        GpioMode::OutputOpenDrain(speed) => (0b01, mode_speed_bits(speed)),
        GpioMode::AltPushPull(speed) => (0b10, mode_speed_bits(speed)),
        GpioMode::AltOpenDrain(speed) => (0b11, mode_speed_bits(speed)),
    };

    let value = ((cnf_bits << 2) | mode_bits) & 0xF; // dịch 2 bits CNF bits cao cộng (OR) thêm 2 bits MODE và AND với 0xF (0b1111) để tránh tràn sang bit khác
    reg_set_bits(gpio_mode_reg_addr, value, bit_position, 4);
}

fn mode_speed_bits(speed: u8) -> u32 {
    match speed {
        10 => 0b01,
        2 => 0b10,
        50 => 0b11,
        _ => panic!("Invalid speed: must be 10, 2, 50 Mhz"),
    }
}

pub enum PinState {
    High,
    Low,
    Toggle,
}

pub fn set_gpio_pin_state(port: u32, pin: u32, pin_state: PinState) {
    let gpio_bsrr_addr = (port + 0x10) as *mut u32;

    match pin_state {
        PinState::High => {
            reg_set_val(gpio_bsrr_addr, 1 << pin);
        }

        PinState::Low => {
            reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
        }

        PinState::Toggle => {
            let gpio_odr_addr = (port + 0x0C) as *mut u32;
            if reg_read_bit(gpio_odr_addr, pin) {
                reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
            } else {
                reg_set_val(gpio_bsrr_addr, 1 << pin);
            }
        }
    }
}

pub fn get_gpio_pin_state(port: u32, pin: u32) -> bool {
    let gpio_idr_addr = (port + 0x08) as *mut u32;
    reg_read_bit(gpio_idr_addr, pin)
}
