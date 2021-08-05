use crate::hal::{common, gpio, usart};

/* Register Base */
/* Reset and Clock Control (RCC) */
pub const RCC_BASE:                 u32 = 0x40021000;

/* General Purpose I/O */
pub const GPIOA_BASE:               u32 = 0x42020000;  
pub const GPIOB_BASE:               u32 = 0x42020400; 
pub const GPIOC_BASE:               u32 = 0x42020800;
pub const GPIOD_BASE:               u32 = 0x42020C00;
pub const GPIOE_BASE:               u32 = 0x42021000;
pub const GPIOF_BASE:               u32 = 0x42021400;
pub const GPIOG_BASE:               u32 = 0x42021800;
pub const GPIOH_BASE:               u32 = 0x42021C00;

/* Timers */
pub const TIMER1_BASE:              u32 = 0x40012C00;
pub const TIMER2_BASE:              u32 = 0x40000000;
pub const TIMER3_BASE:              u32 = 0x40000400;
pub const TIMER4_BASE:              u32 = 0x40000800;
pub const TIMER5_BASE:              u32 = 0x40000C00;
pub const TIMER6_BASE:              u32 = 0x40001000;
pub const TIMER7_BASE:              u32 = 0x40001400;
pub const TIMER8_BASE:              u32 = 0x40013400;
pub const TIMER15_BASE:             u32 = 0x40014000;
pub const TIMER16_BASE:             u32 = 0x40014400;
pub const TIMER17_BASE:             u32 = 0x40014800;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
pub const USART1_BASE:              u32 = 0x40013800;
pub const USART2_BASE:              u32 = 0x40004400;      
pub const USART3_BASE:              u32 = 0x40004800;
pub const USART4_BASE:              u32 = 0x40004C00;
pub const USART5_BASE:              u32 = 0x40005000;

/* Inter-Integrated Circuit (I2C) */
pub const I2C1_BASE:                u32 = 0x40005400; 
pub const I2C2_BASE:                u32 = 0x40005800;
pub const I2C3_BASE:                u32 = 0x40005C00;

/* Serial Peripheral Interface */
pub const SPI1_BASE:                u32 = 0x40013000; 
pub const SPI2_BASE:                u32 = 0x40003800;
pub const SPI3_BASE:                u32 = 0x40003C00;

/* CAN Interface */
pub const CAN_BASE:                 u32 = 0x4000A400;

pub const NVIC_BASE:                u32 = 0xE000E100;
      
/* Reset and Clock Control (RCC) */
pub const RCC_GPIOA_AHB2EN:         u32 = common::BIT_0;
pub const RCC_GPIOB_AHB2EN:         u32 = common::BIT_1;
pub const RCC_GPIOC_AHB2EN:         u32 = common::BIT_2;
pub const RCC_GPIOD_AHB2EN:         u32 = common::BIT_3;
pub const RCC_GPIOE_AHB2EN:         u32 = common::BIT_4;
pub const RCC_GPIOF_AHB2EN:         u32 = common::BIT_5;

/* General Purpose I/O */
/* NUCLEO BOARD PIN OUT SPECIFICS - NUCLEO - L552ZE-Q */
pub const GPIOC_PIN7:               u32 = 7;                             /* USER GREEN LED on GPIO C Bus, Pin 7  */
pub const LED_GRN_PIN:              u32 = GPIOC_PIN7;                              /* USER GREEN LED on GPIO C Bus, Pin 7  */
pub const LED_GRN:                  u32 = common::BIT_7;                                   /* USER GREEN LED on GPIO C Bus, Pin 7  */
pub const GPIOB_PIN7:               u32 = 7;                             /* USER BLUE LED on GPIO B Bus, Pin 7   */
pub const LED_BLU_PIN:              u32 = GPIOB_PIN7;                              /* USER BLUE LED on GPIO B Bus, Pin 7   */
pub const LED_BLU:                  u32 = common::BIT_7;                                   /* USER BLUE LED on GPIO B Bus, Pin 7   */
pub const GPIOA_PIN9:               u32 = 9;                             /* USER RED LED on GPIO A Bus, Pin 9    */
pub const LED_RED_PIN:              u32 = GPIOA_PIN9;                              /* USER RED LED on GPIO A Bus, Pin 9    */
pub const LED_RED:                  u32 = common::BIT_9;

/* GPIO SETUP */
pub const USER_LED_MODE:            gpio::Mode = gpio::Mode::Out;
pub const USER_LED_OTYPE:           gpio::OType = gpio::OType::PushPull;
pub const USER_LED_AF:              gpio::AltFunc = gpio::AltFunc::Af0;

/* Timer */
pub const TIMER2_RCC_APB1R1_ENABLE: u32 = common::BIT_0;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
pub const USART2_RCC_APB1R1_ENABLE: u32 = common::BIT_17;
pub const PORTA_PIN2:               u32 = 2;    //A7    TX
pub const PORTA_PIN3:               u32 = 3;    //A2    RX
pub const USART2_TX:                u32 = PORTA_PIN2;
pub const USART2_RX:                u32 = PORTA_PIN3;

/* GPIO SETUP */
pub const USART_MODE:               gpio::Mode = gpio::Mode::Alt;
pub const USART_OTYPE:              gpio::OType = gpio::OType::PushPull;
pub const USART_AF:                 gpio::AltFunc = gpio::AltFunc::Af7;

/* I2C 1*/
pub const I2C1_RCC_APB1R1_ENABLE:   u32 = common::BIT_21;
pub const PORTB_PIN6:               u32 = 6;    //D5    SCL
pub const PORTB_PIN7:               u32 = 7;    //D4    SDA
pub const I2C1_SCL:                 u32 = PORTB_PIN6;
pub const I2C1_SDA:                 u32 = PORTB_PIN7;

/* CAN */
pub const CAN_RCC_APB1R1_ENABLE:    u32 = common::BIT_25;
pub const PORTA_PIN11:              u32 = 11;   //D10   RX
pub const PORTA_PIN12:              u32 = 12;   //D2    TX
pub const CAN_RX:                   u32 = PORTA_PIN11;
pub const CAN_TX:                   u32 = PORTA_PIN12;

/* GPIO SETUP */
pub const CAN_MODE:                 gpio::Mode = gpio::Mode::Alt;
pub const CAN_OTYPE:                gpio::OType = gpio::OType::PushPull;
pub const CAN_AF:                   gpio::AltFunc = gpio::AltFunc::Af9;
pub const CAN_OSPEED:               gpio::OSpeed = gpio::OSpeed::High;
pub const CAN_PUPD:                 gpio::Pupd = gpio::Pupd::Pu;

/* SPI 1*/
/* RCC */
pub const SPI1_RCC_APB2R_ENABLE:    u32 = common::BIT_12;

/* SPI 2*/
/* RCC */
// pub const SPI2_RCC_APB1R1_ENABLE:   u32 = common::BIT_14; // NOT AVAILABLE 432KC

/* SPI 3*/
/* RCC */
pub const SPI3_RCC_APB1R1_ENABLE:   u32 = common::BIT_15;