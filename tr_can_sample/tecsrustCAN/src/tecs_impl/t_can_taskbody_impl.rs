use crate::tecs_global::*;
use crate::tecs_celltype::t_can_taskbody::*;
use crate::tecs_signature::{s_can_measure_for_opt::*, s_task_body::*};

use crate::print;
use crate::tecs_print::*;
use crate::x_can::*;

const N :u32 = 1000;

unsafe extern "C" {
	fn fch_hrt() -> crate::tecs_print::HrtCnt;
	fn loc_cpu() -> itron::abi::ER;
	fn unl_cpu() -> itron::abi::ER;
	fn dis_dsp() -> itron::abi::ER;
	fn ena_dsp() -> itron::abi::ER;
}

impl STaskBody for ETaskbodyForTCanTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

		itron::task::delay(itron::time::duration!(ms: 100)).expect("delay failed");

		print!("Start CAN loopback evaluation",);

		lg.c_can.loopback_setup();

		let test_message_id = 1024;
		let frame_data_length = 8;
		let mut tx_frame: [u32; 16] = [0; 16];
		let mut rx_frame: [u32; 16] = [0; 16];

		tx_frame[0] = x_can_create_id_value(test_message_id, 0, 0, 0, 0);
		tx_frame[1] = x_can_create_dlc_value(frame_data_length);

		{
			let frame_data = unsafe {
				core::slice::from_raw_parts_mut(
					tx_frame.as_mut_ptr().add(2) as *mut u8,
					frame_data_length as usize,
				)
			};

			for (index, byte) in frame_data.iter_mut().enumerate() {
				*byte = index as u8;
			}
		}

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

			let tx_result = lg.c_can.send(&tx_frame);

			unsafe{ 
				end_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				end_l  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}

			itron::task::delay(itron::time::duration!(ms: 1)).expect("delay failed");

			// unsafe{ 
			// 	start_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
			// 	start_l  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			// }

			let rx_result = lg.c_can.receive(&mut rx_frame);

			// unsafe{ 
			// 	end_u1 = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
			// 	end_l  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			// }

			let cnt64_start = ((start_u1 as u64) << 32) | (start_l as u64);
			start = cnt64_start as crate::tecs_print::HrtCnt;

			let cnt64_end = ((end_u1 as u64) << 32) | (end_l as u64);
			end = cnt64_end as crate::tecs_print::HrtCnt;

			if (end - start - overhead) > 0 {
				duration = end - start - overhead;
				print!("%tu,", duration );
			} else {
				print!("duration is negative,", );
			}

			match tx_result {
				Ok(_) => {
					// Successfully sent
				}
				Err(_) => {
					print!("failure: send", );
				}
			}

			match rx_result {
				Ok(_) => {
					if rx_frame[0] != x_can_create_id_value(test_message_id, 0, 0, 0, 0) {
						print!("Loop back error: Invalide ID",);
					}
			
					if (rx_frame[1] >> XCAN_DLCR_DLC_SHIFT) != (x_can_create_dlc_value(frame_data_length) >> XCAN_DLCR_DLC_SHIFT) {
						print!("Loop back error: Invalide DLC",);
					}
		
					let frame_data = unsafe {
						core::slice::from_raw_parts(
							rx_frame.as_ptr().add(2) as *const u8,
							frame_data_length as usize,
						)
					};
		
					for (index, &byte) in frame_data.iter().enumerate() {
						if byte != index as u8 {
							print!("Loopback error: Invalid data",);
						}
					}
				}
				Err(_) => {
					print!("failure: receive", );
				}
			}

			itron::task::delay(itron::time::duration!(ms: 10)).expect("delay failed");
		}

		print!("Finish evaluation",);
		loop {}
	}
}

