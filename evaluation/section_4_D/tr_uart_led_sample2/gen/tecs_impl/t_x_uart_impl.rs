use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart::*;
use crate::tecs_signature::{si_sio_cbr::*, s_x_uart_measure::*, si_handler_body::*};
impl SXUartMeasure for EXUartForTXUart{

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

impl SiHandlerBody for EiHandlerBodyForTXUart{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
}

