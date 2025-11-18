use crate::{t_x_uart::*, si_sio_cbr::*, s_x_uart_measure::*, si_handler_body::*};

use core::ptr::{write_volatile, read_volatile};

use crate::print;
use crate::tecs_print::*;
use crate::xuart::*;

impl SXUartMeasure for EXUartForTXUart<'_>{

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

			write_volatile((*lg.base_address + XUART_IER_OFFSET) as *mut u32, XUART_IXR_TXEMPTY);
			write_volatile((*lg.base_address + XUART_IER_OFFSET) as *mut u32, XUART_IXR_RXTRIG);

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
	fn get_char(&'static self, c: &mut u8) -> bool{
		let lg = self.cell.get_cell_ref();

		unsafe{
			if((read_volatile((*lg.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_RXEMPTY) == 0x00)
			{
				*c = read_volatile((*lg.base_address + XUART_FIFO_OFFSET) as *const u32) as u8;
				return true;
			}
		}

		return false;
	}
}

impl SiHandlerBody for EiHandlerBodyForTXUart<'_>{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();
		// 受信
		unsafe{
			// if (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_RXTRIG) == 1 {
			// 	while (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_SR_RXEMPTY) == 0 {
			// 		c_x_uart_main.ready_receive();
			// 	}
			// }

			if (read_volatile((*lg.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_RXEMPTY) == 0 {
				lg.c_x_uart_main.ready_receive();
			}

			write_volatile((*lg.base_address + XUART_ISR_OFFSET) as *mut u32, (read_volatile((*lg.base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_RXTRIG));
		}

		// 送信
		unsafe{
			// if (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_TXEMPTY) == 1 {
			// 	while (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0 {
			// 		c_x_uart_main.ready_send();
			// 	}
			// }

			if (read_volatile((*lg.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0 {
				lg.c_x_uart_main.ready_send();
			}

			write_volatile((*lg.base_address + XUART_ISR_OFFSET) as *mut u32, (read_volatile((*lg.base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_TXEMPTY));
		}
	}
}

