use crate::{t_semaphore_rs::*, s_semaphore_rs::*, si_semaphore::*, si_notification_handler::*};

impl SSemaphoreRs for ESemaphoreForTSemaphoreRs<'_>{

	#[inline]
	fn signal(&'static self) -> Result<(), Error<SignalError>>{
		let semaphore_ref = self.cell.get_cell_ref();

	}
	#[inline]
	fn wait(&'static self) -> Result<(), Error<WaitError>>{
		let semaphore_ref = self.cell.get_cell_ref();

	}
	#[inline]
	fn wait_polling(&'static self) -> Result<(), Error<PollError>>{
		let semaphore_ref = self.cell.get_cell_ref();

	}
	#[inline]
	fn wait_timeout(&'static self, timeout: &Timeout) -> Result<(), Error<WaitTimeoutError>>{
		let semaphore_ref = self.cell.get_cell_ref();

	}
	#[inline]
	fn initialize(&'static self) -> Result<(), Error<InitializeError>>{
		let semaphore_ref = self.cell.get_cell_ref();

	}
	#[inline]
	fn refer(&'static self) -> Result<Info, Error<InfoError>>{
		let semaphore_ref = self.cell.get_cell_ref();

	}
}

impl SiSemaphore for EiSemaphoreForTSemaphoreRs<'_>{

	#[inline]
	fn signal(&'static self) -> ER{
		let semaphore_ref = self.cell.get_cell_ref();

	}
}

impl SiNotificationHandler for EiNotificationHandlerForTSemaphoreRs<'_>{

}

