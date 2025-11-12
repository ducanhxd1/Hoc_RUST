// File nay sẽ chứa các địa chỉ cụ thể (base address of peripherals and clock speed and other details)

// APB1_BASE
pub const APB1_BASE: u32 = 0x4000_0000; // TIM2
pub const TIM3_BASE: u32 = APB1_BASE + 0x0400;
pub const TIM4_BASE: u32 = APB1_BASE + 0x0800;
pub const TIM5_BASE: u32 = APB1_BASE + 0x0C00;
pub const TIM6_BASE: u32 = APB1_BASE + 0x1000;

pub const SPI2_BASE: u32 = APB1_BASE + 0x3800;
pub const SPI3_BASE: u32 = APB1_BASE + 0x3C00;

pub const USART2_BASE: u32 = APB1_BASE + 0x4400;
pub const USART3_BASE: u32 = APB1_BASE + 0x4800;
pub const UART4_BASE: u32 = APB1_BASE + 0x4C00;
pub const UART5_BASE: u32 = APB1_BASE + 0x5000;

pub const I2C1_BASE: u32 = APB1_BASE + 0x5400;
pub const I2C2_BASE: u32 = APB1_BASE + 0x5800;

// APB2_BASE
pub const APB2_BASE: u32 = 0x4001_0000;
pub const AFIO_BASE: u32 = APB2_BASE; // AFIO BASE SYSCFG_BASE
pub const EXTI_BASE: u32 = APB2_BASE + 0x0400;

pub const GPIOA_BASE: u32 = APB2_BASE + 0x0800;
pub const GPIOB_BASE: u32 = APB2_BASE + 0x0C00;
pub const GPIOC_BASE: u32 = APB2_BASE + 0x1000;
pub const GPIOD_BASE: u32 = APB2_BASE + 0x1400;
pub const GPIOE_BASE: u32 = APB2_BASE + 0x1800;
pub const GPIOF_BASE: u32 = APB2_BASE + 0x1C00;
pub const GPIOG_BASE: u32 = APB2_BASE + 0x2000;

pub const ADC1_BASE: u32 = APB2_BASE + 0x2400;
pub const ADC2_BASE: u32 = APB2_BASE + 0x2800;
pub const ADC3_BASE: u32 = APB2_BASE + 0x3C00;

pub const TIM1_BASE: u32 = APB2_BASE + 0x2C00;
pub const TIM8_BASE: u32 = APB2_BASE + 0x3400;

pub const SPI1_BASE: u32 = APB2_BASE + 0x3000;

pub const USART1_BASE: u32 = APB2_BASE + 0x3800;

// AHB
pub const AHB_BASE: u32 = 0x4001_8000;
pub const RCC_BASE: u32 = 0x4002_1000;

pub const GPIO_PIN_0: u32 = 0;
pub const GPIO_PIN_1: u32 = 1;
pub const GPIO_PIN_2: u32 = 2;
pub const GPIO_PIN_3: u32 = 3;
pub const GPIO_PIN_4: u32 = 4;
pub const GPIO_PIN_5: u32 = 5;
pub const GPIO_PIN_6: u32 = 6;
pub const GPIO_PIN_7: u32 = 7;
pub const GPIO_PIN_8: u32 = 8;
pub const GPIO_PIN_9: u32 = 9;
pub const GPIO_PIN_10: u32 = 10;
pub const GPIO_PIN_11: u32 = 11;
pub const GPIO_PIN_12: u32 = 12;
pub const GPIO_PIN_13: u32 = 13;
pub const GPIO_PIN_14: u32 = 14;
pub const GPIO_PIN_15: u32 = 15;

// STM32F103 

#[allow(non_camel_case_types)]

pub enum IRQn {
    WWDG = 0,            // Window WatchDog Interrupt
    PVD = 1,             // PVD through EXTI Line detection Interrupt
    TAMPER = 2,          // Tamper Interrupt
    RTC = 3,             // RTC global Interrupt
    FLASH = 4,           // Flash global Interrupt
    RCC = 5,             // RCC global Interrupt
    EXTI0 = 6,           // EXTI Line0 Interrupt
    EXTI1 = 7,           // EXTI Line1 Interrupt
    EXTI2 = 8,           // EXTI Line2 Interrupt
    EXTI3 = 9,           // EXTI Line3 Interrupt
    EXTI4 = 10,          // EXTI Line4 Interrupt
    DMA1_Channel1 = 11,  // DMA1 Channel1 Interrupt
    DMA1_Channel2 = 12,  // DMA1 Channel2 Interrupt
    DMA1_Channel3 = 13,  // DMA1 Channel3 Interrupt
    DMA1_Channel4 = 14,  // DMA1 Channel4 Interrupt
    DMA1_Channel5 = 15,  // DMA1 Channel5 Interrupt
    DMA1_Channel6 = 16,  // DMA1 Channel6 Interrupt
    DMA1_Channel7 = 17,  // DMA1 Channel7 Interrupt
    ADC1_2 = 18,         // ADC1 and ADC2 Interrupts
    USB_HP_CAN_TX = 19,  // USB High Priority or CAN TX
    USB_LP_CAN_RX0 = 20, // USB Low Priority or CAN RX0
    CAN_RX1 = 21,        // CAN RX1
    CAN_SCE = 22,        // CAN SCE
    EXTI9_5 = 23,        // EXTI Lines 9 to 5
    TIM1_BRK = 24,       // TIM1 Break
    TIM1_UP = 25,        // TIM1 Update
    TIM1_TRG_COM = 26,   // TIM1 Trigger/Commutation
    TIM1_CC = 27,        // TIM1 Capture Compare
    TIM2 = 28,           // TIM2 global
    TIM3 = 29,           // TIM3 global
    TIM4 = 30,           // TIM4 global
    I2C1_EV = 31,        // I2C1 Event
    I2C1_ER = 32,        // I2C1 Error
    I2C2_EV = 33,        // I2C2 Event
    I2C2_ER = 34,        // I2C2 Error
    SPI1 = 35,           // SPI1 global
    SPI2 = 36,           // SPI2 global
    USART1 = 37,         // USART1 global
    USART2 = 38,         // USART2 global
    USART3 = 39,         // USART3 global
    EXTI15_10 = 40,      // EXTI Lines 15..10
    RTCAlarm = 41,       // RTC Alarm through EXTI Line
    USBWakeUp = 42,      // USB Wakeup from suspend
}

impl IRQn {
    /// Trả về IRQn tương ứng với số pin EXTI
    pub fn from_pin(pin: u32) -> Option<u32> {
        match pin {
            0 => Some(IRQn::EXTI0 as u32),
            1 => Some(IRQn::EXTI1 as u32),
            2 => Some(IRQn::EXTI2 as u32),
            3 => Some(IRQn::EXTI3 as u32),
            4 => Some(IRQn::EXTI4 as u32),
            5..=9 => Some(IRQn::EXTI9_5 as u32),
            10..=15 => Some(IRQn::EXTI15_10 as u32),
            _ => None,
        }
    }
}
