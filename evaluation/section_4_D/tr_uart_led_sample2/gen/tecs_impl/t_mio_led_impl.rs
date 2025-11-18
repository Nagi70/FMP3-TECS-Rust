use crate::tecs_global::*;
use crate::tecs_celltype::t_mio_led::*;
use crate::tecs_signature::s_led::*;
impl SLed for ELedForTMioLed{

	#[inline]
	fn set_up(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn light_on(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn light_off(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
}

