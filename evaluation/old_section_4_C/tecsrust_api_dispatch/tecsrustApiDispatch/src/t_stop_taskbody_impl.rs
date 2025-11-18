use crate::{t_stop_taskbody::*, s_task_rs::*, s_task_body::*};
use crate::print;
use crate::tecs_print::*;
use core::num::NonZeroI32;

use itron::abi::*;
use itron::task::*;
use itron::task::State::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

use crate::t_measure_impl::*;

use itron::time::{duration, Duration, timeout, Timeout};

extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

impl STaskBody for ETaskbodyForTStopTaskbody<'_>{

	fn main(&'static self) {
		let (c_self_task, api) = self.cell.get_cell_ref();

		unsafe{
			END = fch_hrt();
		}

		unsafe{
			let duration = END - START - OVERHEAD;
			print!("%tu,", duration);
		}

		if(*api == 2){
			let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());
			c_self_task.migrate(&processor1);
		}
	}
}

