use crate::{t_xuart::*, si_sio_cbr::*, s_xuart_measure::*, si_handler_body::*};

impl SXuartMeasure for EXuartForTXuart<'_>{

	fn open(&'static self) {
		let (c_xuart_main, base_address, mode, baudgen, bauddiv) = self.cell.get_cell_ref();

	}
	fn put_char(&'static self, c: &u8) -> bool{
		let (c_xuart_main, base_address, mode, baudgen, bauddiv) = self.cell.get_cell_ref();

	}
	fn get_char(&'static self, c: &mut u8) -> bool{
		let (c_xuart_main, base_address, mode, baudgen, bauddiv) = self.cell.get_cell_ref();

	}
}

impl SiHandlerBody for EiHandlerBodyForTXuart<'_>{

	fn main(&'static self) {
		let (c_xuart_main, base_address, mode, baudgen, bauddiv) = self.cell.get_cell_ref();

	}
}

