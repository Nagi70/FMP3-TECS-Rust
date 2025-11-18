use crate::{t_xuart::*, s_xuart_measure::*};

use core::ptr::{write_volatile, read_volatile};

const XUART_CR_OFFSET: u32 = 0x00;
const XUART_MR_OFFSET: u32 = 0x04;
const XUART_IER_OFFSET: u32 = 0x08;
const XUART_IDR_OFFSET: u32 = 0x0c;
const XUART_ISR_OFFSET: u32 = 0x14;
const XUART_BAUDGEN_OFFSET: u32 = 0x18;
const XUART_RXTOUT_OFFSET: u32 = 0x1c;
const XUART_RXWM_OFFSET: u32 = 0x20;
const XUART_SR_OFFSET: u32 = 0x2c;
const XUART_FIFO_OFFSET: u32 = 0x30;
const XUART_BAUDDIV_OFFSET: u32 = 0x34;

const XUART_CR_STOPPRK: u32 = 0x0100;
const XUART_CR_TX_DIS: u32 = 0x0020;
const XUART_CR_TX_EN: u32 = 0x0010;
const XUART_CR_RX_DIS: u32 = 0x0008;
const XUART_CR_RX_EN: u32 = 0x0004;
const XUART_CR_TXRST: u32 = 0x0002;
const XUART_CR_RXRST: u32 = 0x0001;

const XUART_MR_STOPBIT_1: u32 = 0x0000;
const XUART_MR_PARITY_NONE: u32 = 0x0020;
const XUART_MR_CHARLEN_8: u32 = 0x0000;
const XUART_MR_CLKSEL: u32 = 0x0001;
const XUART_MR_CCLK: u32 = 0x0400;

const XUART_IXR_TXEMPTY: u32 = 0x0008;
const XUART_IXR_RXTRIG: u32 = 0x0001;
const XUART_IXR_ALL: u32 = 0x1fff;

const XUART_SR_TXFULL: u32 = 0x0010;
const XUART_SR_TXEMPTY: u32 = 0x0008;
const XUART_SR_RXEMPTY: u32 = 0x0002;

const XUART_BAUDGEN_115K: u32 = 0x7c;
const XUART_BAUDDIV_115K: u32 = 0x06;

impl SXuartMeasure for EXuartForTXuart<'_>{

	fn open(&'static self) {
		let lg = self.cell.get_cell_ref();

		unsafe{
			write_volatile((*lg.base_address + XUART_IDR_OFFSET) as *mut u32, XUART_IXR_ALL);

			write_volatile((*lg.base_address + XUART_ISR_OFFSET) as *mut u32, read_volatile((*lg.base_address + XUART_ISR_OFFSET) as *const u32));

			write_volatile((*lg.base_address + XUART_CR_OFFSET) as *mut u32,
						   XUART_CR_TXRST | XUART_CR_RXRST | XUART_CR_TX_DIS | XUART_CR_RX_DIS);
			
			write_volatile((*lg.base_address + XUART_BAUDGEN_OFFSET) as *mut u32, *lg.baudgen);
			write_volatile((*lg.base_address + XUART_BAUDDIV_OFFSET) as *mut u32, *lg.bauddiv);

			write_volatile((*lg.base_address + XUART_MR_OFFSET) as *mut u32, *lg.mode);

			write_volatile((*lg.base_address + XUART_RXWM_OFFSET) as *mut u32, 0x01);
			
			write_volatile((*lg.base_address + XUART_RXTOUT_OFFSET) as *mut u32, 0x10);

			write_volatile((*lg.base_address + XUART_CR_OFFSET) as *mut u32, XUART_CR_TX_EN | XUART_CR_RX_EN | XUART_CR_STOPPRK);
		}
	}

	fn put_char(&'static self, c: &u8) -> bool{
		let lg = self.cell.get_cell_ref();

		unsafe{
			if((read_volatile((*lg.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0x00)
			{
				write_volatile((*lg.base_address + XUART_FIFO_OFFSET) as *mut u32, *c as u32);
				true
			}else{
				false
			}
		}
	}
}

