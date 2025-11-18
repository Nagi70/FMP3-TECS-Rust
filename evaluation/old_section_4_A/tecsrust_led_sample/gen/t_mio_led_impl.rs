use crate::{t_mio_led::*, s_led::*};

impl SLed for ELedForTMioLed<'_>{

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

