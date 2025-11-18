use crate::{t_loop_taskbody::*, s_task_body::*};
use crate::kernel_cfg::*;

use crate::print;
use crate::tecs_print::*;
use core::num::NonZeroI32;

use itron::abi::*;
use itron::task::*;
use itron::task::State::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

use itron::time::{duration, Duration, timeout, Timeout};

static c_task :TaskRef = unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_1_1).unwrap())};
static c_task2 :TaskRef = unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_2_2).unwrap())};

impl STaskBody for ETaskbodyForTLoopTaskbody<'_>{

	fn main(&'static self) {

		loop{
			// {
			// 	let refer = c_task.info();
			// 	match refer {
			// 		Ok(info) => {
			// 			match info.state() {
			// 				Running => {
			// 					print!("Running", );
			// 				},
			// 				Ready => {
			// 					print!("Ready", );
			// 				},
			// 				Waiting => {
			// 					print!("Waiting", );
			// 				},
			// 				Suspended => {
			// 					print!("Suspended", );
			// 				},
			// 				WaitingSuspended => {
			// 					print!("WaitingSuspended", );
			// 				},
			// 				Dormant => {
			// 					print!("Dormant", );
			// 				},
			// 			}
			// 		},
			// 		Err(_) => {
			// 			print!("info error", );
			// 		},
			// 	}
			// 	let dly_result = delay(duration!(s: 10));
			// }

			// {
			// 	let temp = c_task2.priority().unwrap();
			// 	print!("TASK2_2 priority: %tu", temp);
			// 	let dly_result = delay(duration!(s: 10));
			// }
			
		}

	}
}

