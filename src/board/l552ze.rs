use super::super::hal::{common, gpio, usart};

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
/* LEDS */
pub const GPIOC_PIN7:               u32 = 7;                                /* USER GREEN LED on GPIO C Bus, Pin 7  */
pub const LED_GRN_PIN:              u32 = GPIOC_PIN7;                       /* USER GREEN LED on GPIO C Bus, Pin 7  */
pub const LED_GRN:                  u32 = common::BIT_7;                    /* USER GREEN LED on GPIO C Bus, Pin 7  */
pub const GPIOB_PIN7:               u32 = 7;                                /* USER BLUE LED on GPIO B Bus, Pin 7   */
pub const LED_BLU_PIN:              u32 = GPIOB_PIN7;                       /* USER BLUE LED on GPIO B Bus, Pin 7   */
pub const LED_BLU:                  u32 = common::BIT_7;                    /* USER BLUE LED on GPIO B Bus, Pin 7   */
pub const GPIOA_PIN9:               u32 = 9;                                /* USER RED LED on GPIO A Bus, Pin 9    */
pub const LED_RED_PIN:              u32 = GPIOA_PIN9;                       /* USER RED LED on GPIO A Bus, Pin 9    */
pub const LED_RED:                  u32 = common::BIT_9;                    /* USER RED LED on GPIO A Bus, Pin 9    */

/* TIMER3 PWM CH1 */
pub const GPIOE_PIN3:               u32 = 3;                                /* PWM TIMER 3 on GPIO E Bus, Pin 3   */
pub const TIM3_PWM1_PIN:            u32 = GPIOE_PIN3;                       /* PWM TIMER 3 on GPIO E Bus, Pin 3   */

/* TIMER3 PWM CH2 */
pub const GPIOE_PIN4:               u32 = 4;                                /* PWM TIMER 3 on GPIO E Bus, Pin 4   */
pub const TIM3_PWM2_PIN:            u32 = GPIOE_PIN4;                       /* PWM TIMER 3 on GPIO E Bus, Pin 4   */

/* TIMER3 PWM CH3 */
pub const GPIOE_PIN5:               u32 = 5;                                /* PWM TIMER 3 on GPIO E Bus, Pin 5   */
pub const TIM3_PWM3_PIN:            u32 = GPIOE_PIN5;                       /* PWM TIMER 3 on GPIO E Bus, Pin 5   */ 

/* GPIO SETUP */
pub const USER_LED_MODE:            gpio::Mode = gpio::Mode::Out;
pub const USER_LED_OTYPE:           gpio::OType = gpio::OType::PushPull;
pub const USER_LED_AF:              gpio::AltFunc = gpio::AltFunc::Af0;

pub const PWM_TIM3_MODE:            gpio::Mode = gpio::Mode::Alt;
pub const PWM_TIM3_OTYPE:           gpio::OType = gpio::OType::PushPull;
pub const PWM_TIM3_AF:              gpio::AltFunc = gpio::AltFunc::Af2;


/* Timer */
pub const TIMER2_RCC_APB1R1_ENABLE: u32 = common::BIT_0;
pub const TIMER3_RCC_APB1R1_ENABLE: u32 = common::BIT_1;

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

/* SPI */
pub const SPI_MODE:                 gpio::Mode = gpio::Mode::Alt;
pub const SPI_OTYPE:                gpio::OType = gpio::OType::PushPull;
pub const SPI_AF:                   gpio::AltFunc = gpio::AltFunc::Af5;
pub const PORTB_PIN4:               u32 = 4;
pub const SPI1_MISO:                u32 = PORTB_PIN4;
pub const PORTB_PIN5:               u32 = 5;
pub const SPI1_MOSI:                u32 = PORTB_PIN5;
pub const PORTB_PIN3:               u32 = 3;
pub const SPI1_SCK:                 u32 = PORTB_PIN3;
pub const PORTA_PIN4:               u32 = 4;
pub const SPI1_NSS:                 u32 = PORTA_PIN4;
pub const PORTA_PIN7:               u32 = 7;
pub const SPI1_SS:                  u32 = PORTA_PIN7;
pub const SPI1_SS_MODE:             gpio::Mode = gpio::Mode::In;
pub const SPI1_SS_OTYPE:            gpio::OType = gpio::OType::PushPull;
pub const SPI1_SS_AF:               gpio::AltFunc = gpio::AltFunc::Af0;

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


pub enum NvicIrq {
    WWDG_IRQ,                   /*  0       Window Watchdog */
    PDV_PVM_IRQ,                /*  1       PVD/PVM1/PVM2/PVM3/PVM4 through EXTI lines 16/35/36/37/38 interrupts */
    RTC_IRQ,                    /*  2       RTC secure global interrupts (EXTI line 18) */
    RTC_S_IRQ,                  /*  3       RTC secure global interrupts (EXTI line 18) */
    TAMP_IRQ,                   /*  4       Tamper global interrupt (EXTI line 19) */
    TAMP_S_IRQ,                 /*  5       Tamper secure global interrupt (EXTI line 20) */
    FLASH_IRQ,                  /*  6       Flash memory global interrupt */
    FLASH_S_IRQ,                /*  7       Flash memory secure global interrupt */
    GTZC_IRQ,                   /*  8       TZIC secure global interrupt */
    RCC_IRQ,                    /*  9       RCC global interrupt */
    RCC_S_IRQ,                  /*  10      RCC secure global interrupt */
    EXTI0_IRQ,                  /*  11      EXTI Line0 interrupt */ 
    EXTI1_IRQ,                  /*  12      EXTI Line1 interrupt */
    EXTI2_IRQ,                  /*  13      EXTI Line2 interrupt */
    EXTI3_IRQ,                  /*  14      EXTI Line3 interrupt */
    EXTI4_IRQ,                  /*  15      EXTI Line4 interrupt */
    EXTI5_IRQ,                  /*  16      EXTI Line5 interrupt */
    EXTI6_IRQ,                  /*  17      EXTI Line6 interrupt */
    EXTI7_IRQ,                  /*  18      EXTI Line7 interrupt */
    EXTI8_IRQ,                  /*  19      EXTI Line8 interrupt */
    EXTI9_IRQ,                  /*  20      EXTI Line9 interrupt */
    EXTI10_IRQ,                 /*  21      EXTI Line10 interrupt */
    EXTI11_IRQ,                 /*  22      EXTI Line11 interrupt */
    EXTI12_IRQ,                 /*  23      EXTI Line12 interrupt */
    EXTI13_IRQ,                 /*  24      EXTI Line13 interrupt */
    EXTI14_IRQ,                 /*  25      EXTI Line14 interrupt */
    EXTI15_IRQ,                 /*  26      EXTI Line15 interrupt */
    DMAMUX1_IRQ,                /*  27      DMAMUX1 non-secure interrupt */
    DMAMUX1_S_IRQ,              /*  28      DMAMUX1 secure interrupt */
    DMA1_Channel1_IRQ,          /*  29      DMA1 Channel 1 interrupt */
    DMA1_Channel2_IRQ,          /*  30      DMA1 Channel 2 interrupt */
    DMA1_Channel3_IRQ,          /*  31      DMA1 Channel 3 interrupt */
    DMA1_Channel4_IRQ,          /*  32      DMA1 Channel 4 interrupt */
    DMA1_Channel5_IRQ,          /*  33      DMA1 Channel 5 interrupt */
    DMA1_Channel6_IRQ,          /*  34      DMA1 Channel 6 interrupt */
    DMA1_Channel7_IRQ,          /*  35      DMA1 Channel 7 interrupt */
    DMA1_Channel8_IRQ,          /*  36      DMA1 Channel 8 interrupt */
    ADC1_2_IRQ,                 /*  37      ADC1 & ADC2 interrupt */
    DAC_IRQ,                    /*  38      DAC1&2 underrun errors interrupt */
    FDCAN1_IT0_IRQ,             /*  39      FDCAN1 Interrupt 0 interrupt */
    FDCAN1_IT1_IRQ,             /*  40      FDCAN1 Interrupt 1 interrupt */
    TIM1_BRK_IRQ,               /*  41      TIM1 Break interrupt */
    TIM1_UP_IRQ,                /*  42      TIM1 Update interrupt */
    TIM1_TRG_COM_IRQ,           /*  43      TIM1 Trigger and Commutation interrupt */
    TIM1_CC_IRQ,                /*  44      TIM1 Capture Compare interrupt */
    TIM2_IRQ,                   /*  45      TIM2 interrupt */
    TIM3_IRQ,                   /*  46      TIM3 interrupt */
    TIM4_IRQ,                   /*  47      TIM4 interrupt */
    TIM5_IRQ,                   /*  48      TIM5 interrupt */
    TIM6_IRQ,                   /*  49      TIM6 interrupt */
    TIM7_IRQ,                   /*  50      TIM7 interrupt */
    TIM8_BRK_IRQ,               /*  51      TIM8 Break interrupt */
    TIM8_UP_IRQ,                /*  52      TIM8 Update interrupt */
    TIM8_TRG_COM_IRQ,           /*  53      TIM8 Trigger and Commutation interrupt */
    TIM8_CC_IRQ,                /*  54      TIM8 Capture Compare interrupt */
    I2C1_EV_IRQ,                /*  55      I2C1 Event interrupt */
    I2C1_ER_IRQ,                /*  56      I2C1 Error interrupt */
    I2C2_EV_IRQ,                /*  57      I2C2 Event interrupt */
    I2C2_ER_IRQ,                /*  58      I2C2 Error interrupt */
    SPI1_IRQ,                   /*  59      SPI1 interrupt */
    SPI2_IRQ,                   /*  60      SPI2 interrupt */
    USART1_IRQ,                 /*  61      USART1 interrupt */
    USART2_IRQ,                 /*  62      USART2 interrupt */
    USART3_IRQ,                 /*  63      USART3 interrupt */
    UART4_IRQ,                  /*  64      UART4 interrupt */
    UART5_IRQ,                  /*  65      UART5 interrupt */
    LPUART1_IRQ,                /*  66      LP UART1 interrupt */
    LPTIM1_IRQ,                 /*  67      LP TIM1 interrupt */
    LPTIM2_IRQ,                 /*  68      LP TIM2 interrupt */
    TIM15_IRQ,                  /*  69      TIM15 interrupt */
    TIM16_IRQ,                  /*  70      TIM16 interrupt */
    TIM17_IRQ,                  /*  71      TIM17 interrupt */
    COMP_IRQ,                   /*  72      COMP1&2 interrupt */
    USB_FS_IRQ,                 /*  73      USB FS interrupt */
    CRS_IRQ,                    /*  74      CRS interrupt */
    FMC_IRQ,                    /*  75      FMC interrupt */
    OCTOSPI1_IRQ,               /*  76      OctoSPI1 global interrupt */
    SDMMC1_IRQ = 78,            /*  78      SDMMC1 interrupt */
    DMA2_Channel1_IRQ,          /*  80      DMA2 Channel 1 interrupt */
    DMA2_Channel2_IRQ,          /*  81      DMA2 Channel 2 interrupt */
    DMA2_Channel3_IRQ,          /*  82      DMA2 Channel 3 interrupt */
    DMA2_Channel4_IRQ,          /*  83      DMA2 Channel 4 interrupt */
    DMA2_Channel5_IRQ,          /*  84      DMA2 Channel 5 interrupt */
    DMA2_Channel6_IRQ,          /*  85      DMA2 Channel 6 interrupt */
    DMA2_Channel7_IRQ,          /*  86      DMA2 Channel 7 interrupt */
    DMA2_Channel8_IRQ,          /*  87      DMA2 Channel 8 interrupt */
    I2C3_EV_IRQ,                /*  88      I2C3 event interrupt */
    I2C3_ER_IRQ,                /*  89      I2C3 error interrupt */
    SAI1_IRQ,                   /*  90      Serial Audio Interface 1 global interrupt */
    SAI2_IRQ,                   /*  91      Serial Audio Interface 2 global interrupt */
    TSC_IRQ,                    /*  92      Touch Sense Controller global interrupt */
    RNG_IRQ = 94,               /*  94      RNG global interrupt */
    FPU_IRQ,                    /*  95      FPU interrupt */
    HASH_IRQ,                   /*  96      HASH interrupt */
    LPTIM3_IRQ = 98,            /*  98      LP TIM3 interrupt */
    SPI3_IRQ,                   /*  99      SPI3 interrupt */
    I2C4_ER_IRQ,                /*  100     I2C4 error interrupt */
    I2C4_EV_IRQ,                /*  101     I2C4 event interrupt */
    DFSDM1_FLT0_IRQ,            /*  102     DFSDM1 Filter 0 global interrupt */
    DFSDM1_FLT1_IRQ,            /*  103     DFSDM1 Filter 1 global interrupt */
    DFSDM1_FLT2_IRQ,            /*  104     DFSDM1 Filter 2 global interrupt */
    DFSDM1_FLT3_IRQ,            /*  105     DFSDM1 Filter 3 global interrupt */
    UCPD1_IRQ,                  /*  106     UCPD1 interrupt */
    ICACHE_IRQ,                 /*  107     ICACHE interrupt */
}