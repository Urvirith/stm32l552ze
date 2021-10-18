use super::super::hal::spi;

pub struct W5200 {
    cr1:        *mut u32,       // Control Register 1
    cr2:        *mut u32,       // Control Register 2
    sr:         *mut u32,       // Own Address Register 1
    dr:         *mut u32,       // Own Address Register 2
    crcpr:      *mut u32,       // Timing Register
    rxcrcpr:    *mut u32,       // Timeout Register
    txcrcpr:    *mut u32,       // Interrupt And Status Register
}

struct W5200IpAddress {
    ip:                 [u16; 4],
    subnet:             [u16; 4],
    gateway:            [u16; 4],
    mac:                [u16; 6]
}

/* SPI Conditions */
pub const CLK_SETUP:        spi::ClockSetup =   spi::ClockSetup::RisingEdgeClockLow;
pub const BIT_SETUP:        spi::BitFirst =     spi::BitFirst::Msb;
pub const WORD_SETUP:       spi::DataSize =     spi::DataSize::Bits8;


/* Common Register */
/* Mode */
const MODER:            u16 =   0x0000;

/* Gateway Address Register */
/* Example (GAR0 (0xC0 = 192), GAR1 (0xA8 = 168), GAR2 (0x00 = 0), GAR3 (0x01 = 1) -> 192.168.0.1) */
const GAR0:             u16 =   0x0001;
const GAR1:             u16 =   0x0002;
const GAR2:             u16 =   0x0003;
const GAR3:             u16 =   0x0004;

/* Subnet Mask Address Register */
/* Example (SUBR0 (0xFF = 255), SUBR1 (0xFF = 255), SUBR2 (0xFF = 255), SUBR3 (0x00 = 0) -> 255.255.255) */
const SUBR0:            u16 =   0x0005;
const SUBR1:            u16 =   0x0006;
const SUBR2:            u16 =   0x0007;
const SUBR3:            u16 =   0x0008;

/* Source Hardware Address Register */
/* Example (SHAR0 (0x00), SHAR1 (0x08), SHAR2 (0xDC), SHAR3 (0x01), SHAR4 (0x02), SHAR5 (0x03) -> 00:08:DC:01:02:03) */
const SHAR0:            u16 =   0x0009;
const SHAR1:            u16 =   0x000A;
const SHAR2:            u16 =   0x000B;
const SHAR3:            u16 =   0x000C;
const SHAR4:            u16 =   0x000D;
const SHAR5:            u16 =   0x000E;

/* Source IP Address Register */
/* Example (SIPR0 (0xC0 = 192), SIPR1 (0xA8 = 168), SIPR2 (0x00 = 0), SIPR3 (0x02 = 2) -> 192.168.0.2) */
const SIPR0:            u16 =   0x000F;
const SIPR1:            u16 =   0x0010;
const SIPR2:            u16 =   0x0011;
const SIPR3:            u16 =   0x0012;

/* Interrupt Register */
const IR:               u16 =   0x0015;

/* Interrupt Mask Register */
const IMR:              u16 =   0x0016;

/* Retry Time Register */
const RTR0:             u16 =   0x0017;
const RTR1:             u16 =   0x0018;

/* Retry Count Register */
const RCR:              u16 =   0x0019;

/* Chip Version Register */
const VERR:             u16 =   0x001F;

/* Socket Interrupt Register */
const IR2:              u16 =   0x0034;

/* Physical Status Register */
const PSTATUS:          u16 =   0x0035;

/* Socket Interrupt Register */
const IMR2:             u16 =   0x0036;

/* Socket Registers */
/* Register Offset */
const SN_OFFSET:        u16 =   0x0100;

/* Mode */
const SN_MODER:         u16 =   0x4000;

/* Command */
const SN_CR:            u16 =   0x4001;

/* Interrupt */
const SN_IR:            u16 =   0x4002;

/* Status */
const SN_SR:            u16 =   0x4003;

/* Source Port */
const SN_SPORTR0:       u16 =   0x4004;
const SN_SPORTR1:       u16 =   0x4005;

/* Destination Hardware Address Register */
/* Example (SHAR0 (0x00), SHAR1 (0x08), SHAR2 (0xDC), SHAR3 (0x01), SHAR4 (0x02), SHAR5 (0x03) -> 00:08:DC:01:02:03) */
const SN_DHAR0:         u16 =   0x4006;
const SN_DHAR1:         u16 =   0x4007;
const SN_DHAR2:         u16 =   0x4008;
const SN_DHAR3:         u16 =   0x4009;
const SN_DHAR4:         u16 =   0x400A;
const SN_DHAR5:         u16 =   0x400B;

/* Destination IP Address Register */
/* Example (SIPR0 (0xC0 = 192), SIPR1 (0xA8 = 168), SIPR2 (0x00 = 0), SIPR3 (0x02 = 2) -> 192.168.0.2) */
const SN_DIPR0:         u16 =   0x000C;
const SN_DIPR1:         u16 =   0x001D;
const SN_DIPR2:         u16 =   0x001E;
const SN_DIPR3:         u16 =   0x001F;

/* Destination Port */
const SN_DPORTR0:       u16 =   0x4010;
const SN_DPORTR1:       u16 =   0x4011;

/* Maximum Segment Size */
const SN_MSSR0:         u16 =   0x4012;
const SN_MSSR1:         u16 =   0x4013;

/* Protocol In IP Raw Mode */
const SN_PROTO:         u16 =   0x4014;

/* Recieve Memory Size */
const SN_RXMEM_SIZE:    u16 =   0x401E;

/* Transmit Memory Size */
const SN_TXMEM_SIZE:    u16 =   0x401F;

/* TX Socket Free Size */
const SN_TX_FSR0:       u16 =   0x4020;
const SN_TX_FSR1:       u16 =   0x4021;

/* TX Socket Read Pointer */
const SN_TX_RD0:        u16 =   0x4022;
const SN_TX_RD1:        u16 =   0x4023;

/* TX Socket Write Pointer */
const SN_TX_WR0:        u16 =   0x4024;
const SN_TX_WR1:        u16 =   0x4025;

/* RX Socket Free Size */
const SN_RX_FSR0:       u16 =   0x4026;
const SN_RX_FSR1:       u16 =   0x4027;

/* RX Socket Read Pointer */
const SN_RX_RD0:        u16 =   0x4028;
const SN_RX_RD1:        u16 =   0x4029;

/* RX Socket Write Pointer */
const SN_RX_WR0:        u16 =   0x402A;
const SN_RX_WR1:        u16 =   0x402B;

/* Socket Interrupt Mask */
const SN_IMR:           u16 =   0x402C;

/* Socket Fragment Offset In IP Header */
const SN_FRAG0:         u16 =   0x402D;
const SN_FRAG1:         u16 =   0x402E;

