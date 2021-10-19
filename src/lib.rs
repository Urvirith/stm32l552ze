#![no_std] // EMBEDDED PROJECT CORE LIBRARY TO BE USED

use core::panic::PanicInfo;

mod board;
mod hal;
mod axis;
mod driver;

const CLK:                  hal::common::MsiRange = hal::common::MsiRange::Clk16MHz;


#[no_mangle]
pub extern "C" fn _system_init() {
    /* RCC Enabling of the bus */
    let rcc = hal::rcc::Rcc::init(board::l552ze::RCC_BASE);

    rcc.write_msi_range(CLK);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOA_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOB_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOC_AHB2EN);
    rcc.write_ahb2_enr(board::l552ze::RCC_GPIOD_AHB2EN);
    rcc.write_apb1_enr1(board::l552ze::TIMER2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l552ze::TIMER3_RCC_APB1R1_ENABLE);
    rcc.write_apb2_enr(board::l552ze::SPI1_RCC_APB2R_ENABLE);
    rcc.write_apb1_enr1(board::l552ze::USART3_RCC_APB1R1_ENABLE);
}


#[no_mangle]
pub extern "C" fn _start() {
    let freq = hal::common::range(CLK);
    // Initialize the LED on L432KC board
    let gpioa =     hal::gpio::Gpio::init(board::l552ze::GPIOA_BASE);
    let gpiob =     hal::gpio::Gpio::init(board::l552ze::GPIOB_BASE);
    let gpioc =     hal::gpio::Gpio::init(board::l552ze::GPIOC_BASE);
    let gpiod =     hal::gpio::Gpio::init(board::l552ze::GPIOD_BASE);
    let seq_timer = hal::timer::Timer::init(board::l552ze::TIMER2_BASE);
    let int_timer = hal::timer::Timer::init(board::l552ze::TIMER3_BASE);
    let mut nvic =  hal::nvic::Nvic::init(board::l552ze::NVIC_BASE);
    let spi =       hal::spi::Spi::init(board::l552ze::SPI1_BASE);
    let usart =     hal::usart::Usart::init(board::l552ze::USART3_BASE);
    
    /* USART */
    gpiod.otype(board::l552ze::USART3_TX, board::l552ze::USART_MODE, board::l552ze::USART_OTYPE, board::l552ze::USART_AF);
    gpiod.otype(board::l552ze::USART3_RX, board::l552ze::USART_MODE, board::l552ze::USART_OTYPE, board::l552ze::USART_AF);
    usart.open(hal::usart::WordLen::Bits8, hal::usart::StopLen::StopBit1, hal::usart::BaudRate::Baud921600, 16000, hal::usart::OverSample::Oversample16);

    /* SPI 1 Setup */
    gpiob.otype(board::l552ze::SPI1_MISO, board::l552ze::SPI_MODE, board::l552ze::SPI_OTYPE, board::l552ze::SPI_AF);
    gpiob.otype(board::l552ze::SPI1_MOSI, board::l552ze::SPI_MODE, board::l552ze::SPI_OTYPE, board::l552ze::SPI_AF);
    gpiob.otype(board::l552ze::SPI1_SCK, board::l552ze::SPI_MODE, board::l552ze::SPI_OTYPE, board::l552ze::SPI_AF);
    gpioa.otype(board::l552ze::SPI1_NSS, board::l552ze::SPI_MODE, board::l552ze::SPI_OTYPE, board::l552ze::SPI_AF);
    gpioa.otype(board::l552ze::SPI1_SS, board::l552ze::SPI1_SS_MODE, board::l552ze::SPI1_SS_OTYPE, board::l552ze::SPI1_SS_AF);
    spi.open(hal::spi::BaudRateDiv::Clk16, driver::w5200::CLK_SETUP, driver::w5200::BIT_SETUP, driver::w5200::WORD_SETUP);
    /* LED Setup */
    gpioa.otype(board::l552ze::LED_RED_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    gpiob.otype(board::l552ze::LED_BLU_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    gpioc.otype(board::l552ze::LED_GRN_PIN, board::l552ze::USER_LED_MODE, board::l552ze::USER_LED_OTYPE, board::l552ze::USER_LED_AF);
    
    seq_timer.open(hal::timer::TimerType::Cont, hal::timer::Direction::Upcount);
    seq_timer.set_scl(1000, freq, freq);
    seq_timer.start();

    int_timer.open(hal::timer::TimerType::Cont, hal::timer::Direction::Upcount);
    int_timer.set_scl(1000, freq, freq);
    int_timer.set_interrupt();
    int_timer.start();

    nvic.set_interrupt(board::l552ze::NvicIrq::TIM3_IRQ as u32);

    let mut i = 0;
    let mut buf:[u8; 8] = [0x03, 0x01, 0x02, 0x03 ,0x04, 0x05, 0x06, 0x0D];
    let mut spi_obuf:[u8; 4] = [0x03, 0x06, 0x04, 0x0D];
    let mut spi_ibuf:[u8; 4] = [0x00, 0x00, 0x00, 0x00];

    loop {
        if seq_timer.get_flag() { 
            if i == 1 {
                gpiob.set_pin(board::l552ze::LED_BLU);
            } else if i == 2 {
                gpioc.set_pin(board::l552ze::LED_GRN);
            } else {
                gpiob.clr_pin(board::l552ze::LED_BLU);
                gpioc.clr_pin(board::l552ze::LED_GRN);
                i = 0;  
            }

            let len = spi_ibuf.len();
            buf[1] = i;

            spi.enable();
            spi.write(&spi_obuf);
            spi.disable();
            spi.read(&mut spi_ibuf, len);

            spi_obuf[1] = i;

            usart.write(&spi_ibuf);
            usart.write(&buf);


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
    let int_timer = hal::timer::Timer::init(board::l552ze::TIMER3_BASE);

    int_timer.clr_flag();

    if gpioa.get_pin(board::l552ze::LED_RED) {
        gpioa.clr_pin(board::l552ze::LED_RED);
    } else {
        gpioa.set_pin(board::l552ze::LED_RED);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
