use crate::tecs_global::*;
use crate::tecs_celltype::t_semaphore_rs::*;
use crate::tecs_signature::{s_semaphore_rs::*, si_semaphore::*};
impl SSemaphoreRs for ESemaphoreForTSemaphoreRs{

	#[inline]
	fn signal(&self) -> Result<(), itron::error::Error<itron::semaphore::SignalError>>{
		let lg = self.cell.get_cell_ref();
		semaphore_ref.signal()
	}
	#[inline]
	fn wait(&self) -> Result<(), itron::error::Error<itron::semaphore::WaitError>>{
		let lg = self.cell.get_cell_ref();
		semaphore_ref.wait()
	}
	#[inline]
	fn wait_polling(&self) -> Result<(), itron::error::Error<itron::semaphore::PollError>>{
		let lg = self.cell.get_cell_ref();
		semaphore_ref.poll()
	}
	#[inline]
	fn wait_timeout(&self, timeout: &itron::time::Timeout) -> Result<(), itron::error::Error<itron::semaphore::WaitTimeoutError>>{
		let lg = self.cell.get_cell_ref();
		semaphore_ref.wait_timeout(*timeout)
	}
	#[inline]
	fn initialize(&self) -> Result<(), itron::error::Error<itron::semaphore::InitializeError>>{
		let lg = self.cell.get_cell_ref();
		semaphore_ref.initialize()
	}
	#[inline]
	fn refer(&self) -> Result<itron::semaphore::Info, itron::error::Error<itron::semaphore::InfoError>>{
		let lg = self.cell.get_cell_ref();
		semaphore_ref.info()
	}
}

impl SiSemaphore for EiSemaphoreForTSemaphoreRs{

	#[inline]
	fn signal(&self) -> ER{
		let lg = self.cell.get_cell_ref();
		1
	}
}

