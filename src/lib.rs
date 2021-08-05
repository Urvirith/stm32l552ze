#![no_std] // EMBEDDED PROJECT CORE LIBRARY TO BE USED

use core::panic::PanicInfo;

mod board;
mod hal;


const CLK:          hal::common::MsiRange = hal::common::MsiRange::Clk16MHz;

#[no_mangle]
pub extern fn _system_init() {
    /* RCC Enabling of the bus */
    let rcc = hal::rcc::Rcc::init(board::l552ze::RCC_BASE);

    rcc.write_msi_range(CLK);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOA_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOB_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOC_AHB2EN);
    rcc.write_apb1_enr1(board::l552ze::TIMER2_RCC_APB1R1_ENABLE);
}


#[no_mangle]
pub extern fn _start() {
    let freq = hal::common::range(CLK);
    // Initialize the LED on L432KC board
    let gpioa = hal::gpio::Gpio::init(board::l552ze::GPIOA_BASE);
    let gpiob = hal::gpio::Gpio::init(board::l552ze::GPIOB_BASE);
    let gpioc = hal::gpio::Gpio::init(board::l552ze::GPIOC_BASE);
    let seq_timer   = hal::timer::Timer::init(board::l552ze::TIMER2_BASE);
    
    gpioa.otype(board::l552ze::LED_RED_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    gpiob.otype(board::l552ze::LED_BLU_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    gpioc.otype(board::l552ze::LED_GRN_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    
    seq_timer.open(hal::timer::TimerType::Cont, hal::timer::Direction::Upcount);
    seq_timer.set_scl(1000, freq, freq);
    seq_timer.start();

    let mut i = 0;
    loop {
        if seq_timer.get_flag() {
            if i > 7 {
                i = 0;
            }
    
            if i == 1 {
                gpioa.set_pin(board::l552ze::LED_RED);
            } else if i == 2 {
                gpiob.set_pin(board::l552ze::LED_BLU);
            } else if i == 3 {
                gpioc.set_pin(board::l552ze::LED_GRN);
            } else {
                gpioa.clr_pin(board::l552ze::LED_RED);
                gpiob.clr_pin(board::l552ze::LED_BLU);
                gpioc.clr_pin(board::l552ze::LED_GRN);
                i = 0;  
            }
            i += 1;
            seq_timer.clr_flag();
        }
    }
}




#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
