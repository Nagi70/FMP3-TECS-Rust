use crate::{t_self_reset::*, s_task_rs::*, s_task_body::*, s_semaphore_rs::*};
use itron::abi::*;
use itron::task::*;
use itron::task::State::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

use itron::time::{duration, Duration, timeout, Timeout};

impl STaskBody for ESelfResetForTSelfReset<'_>{

	fn main(&'static self) {
		let (c_taskself, c_semaphore) = self.cell.get_cell_ref();

		let mut ter_result :Result<(), Error<TerminateError>> = Ok(());

		let mut sig_result :Result<(), Error<SignalError>> = Ok(());
		let mut wait_result :Result<(), Error<WaitError>> = Ok(());

		// c_task.activate();
		// wait_result = c_semaphore.wait();
		// ter_result = c_taskself.terminate();
		// sig_result = c_semaphore.signal();

		// c_task.get_priority();
		// c_task.set_priority();
		// loop{}
	}
}

