use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart::*;
use crate::tecs_signature::s_x_uart::*;
impl SXUart for EXUartForTXUart{

	fn open(&self) {
		let lg = self.cell.get_cell_ref();

	}
	fn put_char(&self, c: &u8) -> Result<(),()>{
		let lg = self.cell.get_cell_ref();

	}
	fn get_char(&self) -> Result<u8,()>{
		let lg = self.cell.get_cell_ref();

	}
}

