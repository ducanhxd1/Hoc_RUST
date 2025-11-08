
// Steps write startup for stm32f103 ARM Cortex M3
/*
 * 1. Define the vector table for mcu
 * 2. Define the reset handler  
 * 3/ Define the exception handlers
*/
use core::ptr;

// 1. Define the vector table for mcu

#[unsafe(no_mangle)]
extern "C" fn HardFault_Handler() { loop {} }
#[unsafe(no_mangle)]
extern "C" fn NMI_Handler() { loop {} }
// #[unsafe(no_mangle)]
// extern "C" fn Reset_Handler() { loop {} }
#[unsafe(no_mangle)]
extern "C" fn Default_Handler() { loop {} }

unsafe extern "C" {
    unsafe fn BusFault_Handler();
    unsafe fn MemManage_Handler();
    unsafe fn PendSV_Handler();
    unsafe fn SVCall_Handler();
    unsafe fn SysTick_Handler();
    unsafe fn UsageFault_Handler();
    unsafe fn ADC1_2_Handler();
    unsafe fn ADC3_Handler();
    unsafe fn CAN_RX1_Handler();
    unsafe fn CAN_SCE_Handler();
    unsafe fn DMA1_Channel1_Handler();
    unsafe fn DMA1_Channel2_Handler();
    unsafe fn DMA1_Channel3_Handler();
    unsafe fn DMA1_Channel4_Handler();
    unsafe fn DMA1_Channel5_Handler();
    unsafe fn DMA1_Channel6_Handler();
    unsafe fn DMA1_Channel7_Handler();
    unsafe fn DMA2_Channel1_Handler();
    unsafe fn DMA2_Channel2_Handler();
    unsafe fn DMA2_Channel3_Handler();
    unsafe fn DMA2_Channel4_5_Handler();
    unsafe fn EXTI0_Handler();
    unsafe fn EXTI15_10_Handler();
    unsafe fn EXTI1_Handler();
    unsafe fn EXTI2_Handler();
    unsafe fn EXTI3_Handler();
    unsafe fn EXTI4_Handler();
    unsafe fn EXTI9_5_Handler();
    unsafe fn FLASH_Handler();
    unsafe fn FSMC_Handler();
    unsafe fn I2C1_ER_Handler();
    unsafe fn I2C1_EV_Handler();
    unsafe fn I2C2_ER_Handler();
    unsafe fn I2C2_EV_Handler();
    unsafe fn PVD_Handler();
    unsafe fn RCC_Handler();
    unsafe fn RTCAlarm_Handler();
    unsafe fn RTC_Handler();
    unsafe fn SDIO_Handler();
    unsafe fn SPI1_Handler();
    unsafe fn SPI2_Handler();
    unsafe fn SPI3_Handler();
    unsafe fn TAMPER_Handler();
    unsafe fn TIM1_BRK_Handler();
    unsafe fn TIM1_CC_Handler();
    unsafe fn TIM1_TRG_COM_Handler();
    unsafe fn TIM1_UP_Handler();
    unsafe fn TIM2_Handler();
    unsafe fn TIM3_Handler();
    unsafe fn TIM4_Handler();
    unsafe fn TIM5_Handler();
    unsafe fn TIM6_Handler();
    unsafe fn TIM7_Handler();
    unsafe fn TIM8_BRK_Handler();
    unsafe fn TIM8_CC_Handler();
    unsafe fn TIM8_TRG_COM_Handler();
    unsafe fn TIM8_UP_Handler();
    unsafe fn UART4_Handler();
    unsafe fn UART5_Handler();
    unsafe fn USART1_Handler();
    unsafe fn USART2_Handler();
    unsafe fn USART3_Handler();
    unsafe fn USB_HP_CAN_TX_Handler();
    unsafe fn USB_LP_CAN_RX0_Handler();
    unsafe fn WWDG_Handler();
}



#[used]
#[unsafe(link_section = ".isr_vector")]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 75] = [
    Some(Reset_Handler),
    Some(NMI_Handler),
    Some(HardFault_Handler),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMPER_Handler),
    Some(RTC_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Channel1_Handler),
    Some(DMA1_Channel2_Handler),
    Some(DMA1_Channel3_Handler),
    Some(DMA1_Channel4_Handler),
    Some(DMA1_Channel5_Handler),
    Some(DMA1_Channel6_Handler),
    Some(DMA1_Channel7_Handler),
    Some(ADC1_2_Handler),
    Some(USB_HP_CAN_TX_Handler),
    Some(USB_LP_CAN_RX0_Handler),
    Some(CAN_RX1_Handler),
    Some(CAN_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_Handler),
    Some(TIM1_UP_Handler),
    Some(TIM1_TRG_COM_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    Some(USART3_Handler),
    Some(EXTI15_10_Handler),
    Some(RTCAlarm_Handler),
    None,
    Some(TIM8_BRK_Handler),
    Some(TIM8_UP_Handler),
    Some(TIM8_TRG_COM_Handler),
    Some(TIM8_CC_Handler),
    Some(ADC3_Handler),
    Some(FSMC_Handler),
    Some(SDIO_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    Some(UART4_Handler),
    Some(UART5_Handler),
    Some(TIM6_Handler),
    Some(TIM7_Handler),
    Some(DMA2_Channel1_Handler),
    Some(DMA2_Channel2_Handler),
    Some(DMA2_Channel3_Handler),
    Some(DMA2_Channel4_5_Handler),
];

// #[allow(missing_abi)]
unsafe extern "C" {
     static _sidata: u32; /* start of .data section in flash */
     static mut _sdata: u32;  /* start of .data section in RAM */
     static mut _edata: u32;  /* end of .data section in RAM */
     static mut _sbss: u32;   /* start of .bss  in RAM */
     static mut _ebss: u32;   /* end of .bss  in RAM */
}

// 2. Define the reset handler  
#[unsafe(no_mangle)]
extern "C" fn Reset_Handler() {

    // 1. Copy the .data section from FLASH to RAM

    // reference of static variable to C like raw pointer

    // Sử dụng core::ptr thư viện 
    unsafe {
        let mut src_is_flash = ptr::addr_of!(_sidata);
        let mut dest_in_ram = ptr::addr_of_mut!(_sdata);
        let data_end_in_ram = ptr::addr_of_mut!(_edata);

        while dest_in_ram < data_end_in_ram {
            *dest_in_ram = *src_is_flash;
            dest_in_ram = dest_in_ram.add(1);
            src_is_flash = src_is_flash.add(1);
        }

        // 2. Zero out the .bss section in the RAM
        let mut bss = ptr::addr_of_mut!(_sbss);
        let bss_end = ptr::addr_of_mut!(_ebss);

        while bss < bss_end {
            *bss = 0;
            bss = bss.add(1);
        }
    }
        
    // 3. call main() 
    crate::main(); 
} 