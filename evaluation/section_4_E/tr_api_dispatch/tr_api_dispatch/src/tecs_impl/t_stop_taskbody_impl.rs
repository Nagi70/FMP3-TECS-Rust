use crate::tecs_global::*;
use crate::tecs_celltype::t_stop_taskbody::*;
use crate::tecs_signature::{s_task_rs::*, s_task_body::*};

use crate::print;
use crate::tecs_print::*;
use core::num::NonZeroI32;

use itron::abi::*;
use itron::task::*;
use itron::task::State::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

use crate::tecs_impl::t_measure_impl::*;

use itron::time::{duration, Duration, timeout, Timeout};

unsafe extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

impl STaskBody for ETaskbodyForTStopTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

		unsafe{ 
			END_U = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
			END_L  = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
		}

		unsafe{
			let cnt64_start = ((START_U as u64) << 32) | (START_L as u64);
			START = cnt64_start as crate::tecs_print::HrtCnt;
			let cnt64_end = ((END_U as u64) << 32) | (END_L as u64);
			END = cnt64_end as crate::tecs_print::HrtCnt;
		}

		unsafe{
			let duration = END - START - OVERHEAD;
			print!("%tu,", duration);

			// print!("START: %tu", START);
			// print!("END: %tu", END);
		}

		#[cfg(feature = "mig_tsk")]
		{
			let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());
			lg.c_self_task.migrate(&processor1);
		}
	}
}

