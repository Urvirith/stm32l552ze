#![no_std] // EMBEDDED PROJECT CORE LIBRARY TO BE USED

use core::panic::PanicInfo;

mod board;
mod hal;

const CLK:              hal::common::MsiRange = hal::common::MsiRange::Clk16MHz;

static mut TESTSTRUCT:     StructTest = StructTest::init();

struct StructTest {
    direction:  bool,
    state:      u32,
    count:      u32,
    toggle:     bool,
}

impl StructTest {
    pub const fn init() -> StructTest {
        return StructTest {
            direction:  false,
            state:      0,
            count:      0,
            toggle:     false
        }
    }

    pub fn get_toggle(&self) -> bool {
        return self.toggle;
    }

    pub fn set_toggle(&mut self, val: bool) {
        self.toggle = val;
    }
}

#[no_mangle]
pub extern "C" fn _system_init() {
    /* RCC Enabling of the bus */
    let rcc = hal::rcc::Rcc::init(board::l552ze::RCC_BASE);

    rcc.write_msi_range(CLK);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOA_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOB_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOC_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOE_AHB2EN);
    rcc.write_apb1_enr1(board::l552ze::TIMER2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l552ze::TIMER3_RCC_APB1R1_ENABLE);
}


#[no_mangle]
pub extern "C" fn _start() {
    let freq = hal::common::range(CLK);
    // Initialize the LED on L432KC board
    let gpioa =     hal::gpio::Gpio::init(board::l552ze::GPIOA_BASE);
    let gpiob =     hal::gpio::Gpio::init(board::l552ze::GPIOB_BASE);
    let gpioc =     hal::gpio::Gpio::init(board::l552ze::GPIOC_BASE);
    let gpioe =     hal::gpio::Gpio::init(board::l552ze::GPIOE_BASE);
    let seq_timer = hal::timer::Timer::init(board::l552ze::TIMER2_BASE);
    let pwm_timer = hal::timer::Timer::init(board::l552ze::TIMER3_BASE);
    let mut nvic =  hal::nvic::Nvic::init(board::l552ze::NVIC_BASE);
    
    gpioa.otype(board::l552ze::LED_RED_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    gpiob.otype(board::l552ze::LED_BLU_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    gpioc.otype(board::l552ze::LED_GRN_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    
    seq_timer.open(hal::timer::TimerType::Cont, hal::timer::Direction::Upcount);
    seq_timer.set_scl(1000, freq, freq);
    seq_timer.start();

    gpioe.otype(board::l552ze::TIM3_PWM1_PIN, board::l552ze::PWM_TIM3_MODE, board::l552ze::PWM_TIM3_OTYPE, board::l552ze::PWM_TIM3_AF);
    gpioe.otype(board::l552ze::TIM3_PWM2_PIN, board::l552ze::PWM_TIM3_MODE, board::l552ze::PWM_TIM3_OTYPE, board::l552ze::PWM_TIM3_AF);
    gpioe.otype(board::l552ze::TIM3_PWM3_PIN, board::l552ze::PWM_TIM3_MODE, board::l552ze::PWM_TIM3_OTYPE, board::l552ze::PWM_TIM3_AF);

    pwm_timer.open(hal::timer::TimerType::Ons, hal::timer::Direction::Upcount);
    pwm_timer.set_scl(1000, freq, freq);
    pwm_timer.set_pwm_ch1();
    pwm_timer.set_pwm_ch2();
    pwm_timer.set_pwm_ch3();
    pwm_timer.set_pwm_ccr1(500);
    pwm_timer.set_pwm_ccr2(500);
    pwm_timer.set_pwm_ccr3(500);
    pwm_timer.set_interrupt();
    pwm_timer.start();
    nvic.set_interrupt(board::l552ze::NvicIrq::TIM3_IRQ as u32);

    let mut i = 0;
    loop {
        if seq_timer.get_flag() {
            if i > 7 {
                i = 0;
            }
    
            else if i == 1 {
                gpiob.set_pin(board::l552ze::LED_BLU);
            } else if i == 2 {
                gpioc.set_pin(board::l552ze::LED_GRN);
            } else {
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

#[no_mangle]
pub extern "C" fn TIM3_IRQHandler() {
    let gpioa =     hal::gpio::Gpio::init(board::l552ze::GPIOA_BASE);
    let teststruct =   unsafe{&mut TESTSTRUCT};
    let pwm = hal::timer::Timer::init(board::l552ze::TIMER3_BASE);

    pwm.clr_flag();

    if  teststruct.get_toggle() == true {
        gpioa.set_pin(board::l552ze::LED_RED);
        teststruct.set_toggle(false);
    } else {
        gpioa.clr_pin(board::l552ze::LED_RED);
        teststruct.set_toggle(true);
    }

    pwm.start();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
