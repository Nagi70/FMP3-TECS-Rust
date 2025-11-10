use crate::tecs_global::*;
use crate::tecs_celltype::t_led_taskbody::*;
use crate::tecs_signature::{s_led::*, s_task_body::*};

unsafe extern "C" {
	fn fch_hrt() -> crate::tecs_print::HrtCnt;
	fn loc_cpu() -> itron::abi::ER;
	fn unl_cpu() -> itron::abi::ER;
	fn dis_dsp() -> itron::abi::ER;
	fn ena_dsp() -> itron::abi::ER;
}

use crate::print;
use crate::tecs_print::*;

const N: u32 = 1000;

impl STaskBody for ETaskbodyForTLedTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

		print!("Start LED evaluation",);

		lg.c_led.set_up();
		itron::task::delay(itron::time::duration!(ms: 50)).expect("delay failed");

		let mut dispatch_time : crate::tecs_print::HrtCnt = 0;
		let mut dispatch_end : crate::tecs_print::HrtCnt = 0;
		let mut overhead : crate::tecs_print::HrtCnt = 0;

		let mut overhead_start_u1 : u32 = 0;
		let mut overhead_start_l : u32 = 0;
		let mut overhead_start_u2 : u32 = 0;
		let mut overhead_end_u1 : u32 = 0;
		let mut overhead_end_l : u32 = 0;
		let mut overhead_end_u2 : u32 = 0;

		let mut count_overhead : [u32; N as usize] = [0; N as usize];

		unsafe{
			overhead_start_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
			overhead_start_l = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
		}
		for i in 0..N {
			unsafe{
				overhead_end_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				overhead_end_l = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}
		}

		let cnt64_overhead_start = ((overhead_start_u1 as u64) << 32) | (overhead_start_l as u64);
		dispatch_time = cnt64_overhead_start as crate::tecs_print::HrtCnt;

		let cnt64_overhead_end = ((overhead_end_u1 as u64) << 32) | (overhead_end_l as u64);
		dispatch_end = cnt64_overhead_end as crate::tecs_print::HrtCnt;

		overhead = (dispatch_end - dispatch_time) / N;

		print!("Overhead: %tu", overhead);

		for i in 0..N {
			let mut start : crate::tecs_print::HrtCnt = 0;
			let mut end : crate::tecs_print::HrtCnt = 0;
			let mut duration : crate::tecs_print::HrtCnt = 0;

			let mut start_u1 : u32 = 0;
			let mut start_l : u32 = 0;
			
			let mut end_u1 : u32 = 0;
			let mut end_l : u32 = 0;

			unsafe{ 
				start_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				start_l  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}

			lg.c_led.light_on();
			// lg.c_led.light_off();

			unsafe{ 
				end_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				end_l  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}

			itron::task::delay(itron::time::duration!(ms: 10)).expect("delay failed");

			// lg.c_led.light_on();
			lg.c_led.light_off();

			let cnt64_start = ((start_u1 as u64) << 32) | (start_l as u64);
			start = cnt64_start as crate::tecs_print::HrtCnt;

			let cnt64_end = ((end_u1 as u64) << 32) | (end_l as u64);
			end = cnt64_end as crate::tecs_print::HrtCnt;

			if (end - start - overhead) > 0 {
				duration = end - start - overhead;
				print!("%tu,", duration );
			}

			itron::task::delay(itron::time::duration!(ms: 10)).expect("delay failed");
		}

		print!("Finish evaluation",);
		loop {}
	}
}

