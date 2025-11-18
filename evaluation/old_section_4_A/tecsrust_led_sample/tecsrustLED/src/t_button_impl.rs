use crate::{t_button::*, s_button::*};
use core::ptr::{read_volatile};
use crate::print;
use crate::tecs_print::*;

impl SButton for EButtonForTButton<'_>{

	fn is_pushed(&'static self) -> bool{
		let lg = self.cell.get_cell_ref();

		unsafe{
			// let temp = read_volatile(*data_1_ro as *const u32) >> *gpio_offset;
			let temp = read_volatile(*lg.data_1_ro as *const u32);
			print!("\t\t\tDATE_1_RO: %X", temp);
			(temp & 0x01) == 1			
		}
	}
}

