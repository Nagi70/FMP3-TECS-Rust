use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
use crate::tecs_signature::{s_x_uart::*, s_dataqueue_rs::*, s_task_body::*};

use crate::print;
use crate::tecs_print::*;
const N :u64 = 1000;

unsafe extern "C" {
	fn fch_hrt() -> crate::tecs_print::HrtCnt;
	fn loc_cpu() -> itron::abi::ER;
	fn unl_cpu() -> itron::abi::ER;
	fn dis_dsp() -> itron::abi::ER;
	fn ena_dsp() -> itron::abi::ER;
}

impl STaskBody for ETaskbodyForTXUartTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();
		itron::task::delay(itron::time::duration!(s: 1)).expect("delay failed");

		lg.c_x_uart.open();

		let mut dispatch_time : u64 = 0;
		let mut dispatch_end : u64 = 0;
		let mut overhead : u64 = 0;

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
		dispatch_time = cnt64_overhead_start;

		let cnt64_overhead_end = ((overhead_end_u1 as u64) << 32) | (overhead_end_l as u64);
		dispatch_end = cnt64_overhead_end;

		overhead = (dispatch_end - dispatch_time) / N;

		print!("Overhead: %tu", overhead as crate::tecs_print::HrtCnt);

		itron::task::delay(itron::time::duration!(ms: 10)).expect("delay failed");

		for i in 0..N {
			let mut start : u64 = 0;
			let mut end : u64 = 0;
			let mut duration : u64 = 0;

			let mut start_u1 : u32 = 0;
			let mut start_l : u32 = 0;
			let mut start_u2 : u32 = 0;
			
			let mut end_u1 : u32 = 0;
			let mut end_l : u32 = 0;
			let mut end_u2 : u32 = 0;

			unsafe{ 
				start_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				start_l  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}

			let result = lg.c_x_uart.put_char(&b'N');

			unsafe{ 
				end_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				end_l  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}


			let cnt64_start = ((start_u1 as u64) << 32) | (start_l as u64);
			start = cnt64_start;

			let cnt64_end = ((end_u1 as u64) << 32) | (end_l as u64);
			end = cnt64_end;

			if (end - start - overhead) > 0 {
				duration = end - start - overhead;
				print!("%tu,", duration as crate::tecs_print::HrtCnt);
			} else {
				print!("duration is negative",);
			}

			match result {
				Ok(_) => {
					// Successfully sent
				}
				Err(_) => {
					print!("uart false", );
				}
			}
			
			itron::task::delay(itron::time::duration!(ms: 10)).expect("delay failed");
		}
		loop {
		}
	}
}

