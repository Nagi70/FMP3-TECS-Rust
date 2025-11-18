use crate::{t_x_uart::*, si_sio_cbr::*, s_x_uart_measure::*, si_handler_body::*};

impl SXUartMeasure for EXUartForTXUart<'_>{

	fn open(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
	fn put_char(&'static self, c: &u8) -> bool{
		let lg = self.cell.get_cell_ref();

	}
	fn get_char(&'static self, c: &mut u8) -> bool{
		let lg = self.cell.get_cell_ref();

	}
}

impl SiHandlerBody for EiHandlerBodyForTXUart<'_>{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
}

