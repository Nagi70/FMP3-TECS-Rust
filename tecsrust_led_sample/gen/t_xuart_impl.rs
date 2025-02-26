use crate::{t_xuart::*, s_xuart_measure::*};

impl SXuartMeasure for EXuartForTXuart<'_>{

	fn open(&'static self) {
		let (base_address, mode, baudgen, bauddiv) = self.cell.get_cell_ref();

	}
	fn put_char(&'static self, c: &u8) -> bool{
		let (base_address, mode, baudgen, bauddiv) = self.cell.get_cell_ref();

	}
}

