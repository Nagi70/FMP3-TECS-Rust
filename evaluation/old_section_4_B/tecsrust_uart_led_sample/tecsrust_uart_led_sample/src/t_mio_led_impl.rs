use crate::{t_mio_led::*, s_led::*};
use core::ptr::{write_volatile, read_volatile};

impl SLed for ELedForTMioLed<'_>{

	fn set_up(&'static self) {
		let lg = self.cell.get_cell_ref();
		unsafe{
			let dirm = read_volatile(*lg.dirm_0 as *const u32) | (0x01 << 7);
			let oen = read_volatile(*lg.oen_0 as *const u32) | (0x01 << 7);
			write_volatile(*lg.dirm_0 as *mut u32, dirm);
			write_volatile(*lg.oen_0 as *mut u32, oen);
		}
	}
	fn light_on(&'static self) {
		let lg = self.cell.get_cell_ref();
		unsafe{
			let temp = read_volatile(*lg.data_0 as *const u32) | (0x01 << 7);
			write_volatile(*lg.data_0 as *mut u32, temp);
		}
	}
	fn light_off(&'static self) {
		let lg = self.cell.get_cell_ref();
		unsafe{
			let temp = read_volatile(*lg.data_0 as *const u32) & !(0x01 << 7);
			write_volatile(*lg.data_0 as *mut u32, temp);
		}
	}
}

