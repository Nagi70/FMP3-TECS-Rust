use crate::{t_reset::*, s_task_rs::*, s_semaphore_rs::*, s_task_body::*};
use crate::print;
use crate::tecs_print::*;
use core::num::NonZeroI32;
use itron::task::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

use itron::time::{duration, Duration, timeout, Timeout};

impl STaskBody for EResetForTReset<'_>{

	fn main(&'static self) {
		let (c_task, c_taskself, c_taskmig, c_semaphore) = self.cell.get_cell_ref();

		print!("Processor2: TECS/Rust mig_tsk,", );
		delay(duration!(ms: 1000)).expect("delay failed");

		let mut mig_result :Result<(), Error<MigrateError>> = Ok(());
		let mut ter_result :Result<(), Error<TerminateError>> = Ok(());

		let mut sig_result :Result<(), Error<SignalError>> = Ok(());
		let mut wait_result :Result<(), Error<WaitError>> = Ok(());

		let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());

		loop{
			wait_result = c_semaphore.wait();

			// c_taskmig.migrate(&processor2) ↓
			mig_result = c_taskmig.migrate(&processor1);

			// match mig_result {
			// 	Ok(_) => {
			// 		print!("Processor2: mig_tsk succcess", );
			// 	},
			// 	Err(_) => {
			// 	},
			// }

			sig_result = c_semaphore.signal();
		}

	}
}

