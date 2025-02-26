use crate::{t_mio_led::*, s_led::*};
use core::ptr::{write_volatile, read_volatile};

impl SLed for ELedForTMioLed<'_>{

	fn set_up(&'static self) {
		let (data_0, dirm_0, oen_0) = self.cell.get_cell_ref();
		unsafe{
			let dirm = read_volatile(*dirm_0 as *const u32) | (0x01 << 7);
			let oen = read_volatile(*oen_0 as *const u32) | (0x01 << 7);
			write_volatile(*dirm_0 as *mut u32, dirm);
			write_volatile(*oen_0 as *mut u32, oen);
		}
	}
	fn light_on(&'static self) {
		let (data_0, dirm_0, oen_0) = self.cell.get_cell_ref();
		unsafe{
			let temp = read_volatile(*data_0 as *const u32) | (0x01 << 7);
			write_volatile(*data_0 as *mut u32, temp);
		}
	}
	fn light_off(&'static self) {
		let (data_0, dirm_0, oen_0) = self.cell.get_cell_ref();
		unsafe{
			let temp = read_volatile(*data_0 as *const u32) & !(0x01 << 7);
			write_volatile(*data_0 as *mut u32, temp);
		}
	}
}

