pub const XUART_CR_OFFSET: u32 = 0x00;
pub const XUART_MR_OFFSET: u32 = 0x04;
pub const XUART_IER_OFFSET: u32 = 0x08;
pub const XUART_IDR_OFFSET: u32 = 0x0c;
pub const XUART_ISR_OFFSET: u32 = 0x14;
pub const XUART_BAUDGEN_OFFSET: u32 = 0x18;
pub const XUART_RXTOUT_OFFSET: u32 = 0x1c;
pub const XUART_RXWM_OFFSET: u32 = 0x20;
pub const XUART_SR_OFFSET: u32 = 0x2c;
pub const XUART_FIFO_OFFSET: u32 = 0x30;
pub const XUART_BAUDDIV_OFFSET: u32 = 0x34;

pub const XUART_CR_STOPPRK: u32 = 0x0100;
pub const XUART_CR_TX_DIS: u32 = 0x0020;
pub const XUART_CR_TX_EN: u32 = 0x0010;
pub const XUART_CR_RX_DIS: u32 = 0x0008;
pub const XUART_CR_RX_EN: u32 = 0x0004;
pub const XUART_CR_TXRST: u32 = 0x0002;
pub const XUART_CR_RXRST: u32 = 0x0001;

pub const XUART_MR_STOPBIT_1: u32 = 0x0000;
pub const XUART_MR_PARITY_NONE: u32 = 0x0020;
pub const XUART_MR_CHARLEN_8: u32 = 0x0000;
pub const XUART_MR_CLKSEL: u32 = 0x0001;
pub const XUART_MR_CCLK: u32 = 0x0400;


pub const XUART_IXR_TXEMPTY: u32 = 0x0008;
pub const XUART_IXR_RXTRIG: u32 = 0x0001;
pub const XUART_IXR_ALL: u32 = 0x1fff;

pub const XUART_SR_TXFULL: u32 = 0x0010;
pub const XUART_SR_TXEMPTY: u32 = 0x0008;
pub const XUART_SR_RXEMPTY: u32 = 0x0002;

pub const XUART_BAUDGEN_115K: u32 = 0x7c;
pub const XUART_BAUDDIV_115K: u32 = 0x06;