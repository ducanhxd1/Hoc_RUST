use crate::exti;
/* 
    mod keyword is uesd to define or declare a module.
    Modules are a way to organize code into namspaces, 
    making it easier to manage and avoid name conflicts

    Từ khóa 'mod' được sử dụng để định nghĩa hoặc khai báo một mô-đun.
    Mô-đun là một cách để sắp xếp mã thành các không gian tên,
    giúp quản lý dễ dàng hơn và tránh xung đột tên.

    The pub keyword is used to make the module or its items (functions, structs, etc.) public and 
    accessible from outside the module

    Từ khóa pub được sử dụng để làm cho mô-đun hoặc các mục của nó (hàm, cấu trúc, v.v.) trở nên công 
    khai và có thể truy cập từ bên ngoài mô-đun  
*/
use crate::mcu::*;
use crate::reg::*;

pub mod gpio {

    use super::*; // import everthing from parent
    // use crate::reg::reg_set_bit; Hoặc có thể sử dụng cách viết này 
    
    pub enum EdgeTrigger {
        Rising,
        Falling,
    }

    pub fn set_edge(pin: u32, edge: EdgeTrigger) {
        let exti_rtsr_addr = (EXTI_BASE + 0x08) as *mut u32;

        match edge {
            EdgeTrigger::Falling => {
                reg_set_bit(exti_rtsr_addr, pin, true);
            }

            EdgeTrigger::Rising => {
                reg_set_bit(exti_rtsr_addr, pin, true);
            }
        }
    }
    
    // stm32f1 use AFIO, 
    pub fn configure_syscfg(port: u32, pin: u32) {
        let reg_offset = (pin / 4) * 4;
        let bit_position = (pin % 4) * 4;
        let afio_reg_addr = (AFIO_BASE + 0x08 + reg_offset) as *mut u32;

        match port {
            GPIOA_BASE => {
                reg_set_bits(afio_reg_addr, 0, bit_position, 4);
            }

            GPIOB_BASE => {
                reg_set_bits(afio_reg_addr, 1, bit_position, 4);
            }

            GPIOC_BASE => {
                reg_set_bits(afio_reg_addr, 2, bit_position, 4);
            }

            GPIOD_BASE => {
                reg_set_bits(afio_reg_addr, 3, bit_position, 4);
            }

            GPIOE_BASE => {
                reg_set_bits(afio_reg_addr, 4, bit_position, 4);
            }

            GPIOF_BASE => {
                reg_set_bits(afio_reg_addr, 5, bit_position, 4);
            }

            GPIOG_BASE => {
                reg_set_bits(afio_reg_addr, 6, bit_position, 4);
            }
            _ => (),
        }
    }

}

pub enum ExtiLine {
    Line0 = 0,
    Line1,
    Line2,
    Line3,
}

impl ExtiLine {
    // function converts pin to appropriate EXTI line
    pub fn from_pin(pin: u32) -> Option<ExtiLine> {
        match pin {
            0 => Some(ExtiLine::Line0),
            1 => Some(ExtiLine::Line1),
            2 => Some(ExtiLine::Line2),
            3 => Some(ExtiLine::Line3),
            // create similar arms from 0 to 15
            _ => None,
        }
    }
}

fn configure_interrupt(exti_line: ExtiLine, is_enable: bool) {
    // offset of imr exti = 0x00 => = EXTI_BASE 
    let exti_imr_addr = (EXTI_BASE) as *mut u32;
    let line = exti_line as u32;
    reg_set_bit(exti_imr_addr, line, is_enable);
}

pub fn enable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, true);
}

pub fn disable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, false);
}

pub fn clear_peding_interrupt(exti_line: ExtiLine) {
    let exti_pr_addr = (EXTI_BASE + 0x14) as *mut u32;
    let line = exti_line as u32;
    reg_set_bit(exti_pr_addr, line, true);
}