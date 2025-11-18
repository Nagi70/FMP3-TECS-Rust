use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart::*;
use crate::tecs_signature::s_x_uart::*;

use core::ptr::{read_volatile, write_volatile};

impl SXUart for EXUartForTXUart{

	fn open(&self) {
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

			write_volatile((*lg.base_address + XUART_IER_OFFSET) as *mut u32, XUART_IXR_TXEMPTY);
			write_volatile((*lg.base_address + XUART_IER_OFFSET) as *mut u32, XUART_IXR_RXTRIG);

			write_volatile((*lg.base_address + XUART_CR_OFFSET) as *mut u32, XUART_CR_TX_EN | XUART_CR_RX_EN | XUART_CR_STOPPRK);
		}
	}
	fn put_char(&self, c: &u8) -> Result<(),()>{
		let lg = self.cell.get_cell_ref();

		unsafe{
			if((read_volatile((*lg.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0x00)
			{
				write_volatile((*lg.base_address + XUART_FIFO_OFFSET) as *mut u32, *c as u32);
				return Ok(());
			}else{
				return Err(());
			}
		}
	}
	fn get_char(&self) -> Result<u8,()>{
		let lg = self.cell.get_cell_ref();

		unsafe{
			if((read_volatile((*lg.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_RXEMPTY) == 0x00)
			{
				let c = read_volatile((*lg.base_address + XUART_FIFO_OFFSET) as *const u32) as u8;
				return Ok(c);
			}
		}

		return Err(());
	}
}

