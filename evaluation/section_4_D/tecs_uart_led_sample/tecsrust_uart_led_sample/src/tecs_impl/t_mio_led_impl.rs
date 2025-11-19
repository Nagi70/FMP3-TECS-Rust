use crate::tecs_global::*;
use crate::tecs_celltype::t_mio_led::*;
use crate::tecs_signature::s_led::*;
use core::ptr::{write_volatile, read_volatile};

impl SLed for ELedForTMioLed{

	#[inline]
	fn set_up(&self) {
		let lg = self.cell.get_cell_ref();
		unsafe{
			let dirm = read_volatile(*lg.dirm_0 as *const u32) | (0x01 << 7);
			let oen = read_volatile(*lg.oen_0 as *const u32) | (0x01 << 7);
			write_volatile(*lg.dirm_0 as *mut u32, dirm);
			write_volatile(*lg.oen_0 as *mut u32, oen);
		}
	}
	#[inline]
	fn light_on(&self) {
		let lg = self.cell.get_cell_ref();
		unsafe{
			let temp = read_volatile(*lg.data_0 as *const u32) | (0x01 << 7);
			write_volatile(*lg.data_0 as *mut u32, temp);
		}
	}
	#[inline]
	fn light_off(&self) {
		let lg = self.cell.get_cell_ref();
		unsafe{
			let temp = read_volatile(*lg.data_0 as *const u32) & !(0x01 << 7);
			write_volatile(*lg.data_0 as *mut u32, temp);
		}
	}
}

