use crate::tecs_global::*;
use crate::tecs_celltype::t_task_rs::*;
use crate::tecs_signature::{s_task_rs::*, si_task_rs::*, s_task_body::*};
use itron::abi::*;
use itron::task::*;
use itron::error::Error;
use itron::processor::Processor;
impl STaskRs for ETaskForTTaskRs{

	#[inline]
	fn activate(&self) -> Result<(), itron::error::Error<itron::task::ActivateError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.activate()
	}
	#[inline]
	fn migrate_and_activate(&self, prcid: &itron::processor::Processor) -> Result<(), itron::error::Error<itron::task::ActivateOnError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.activate_on(*prcid)
	}
	#[inline]
	fn cancel_activate(&self) -> Result<usize, itron::error::Error<itron::task::CancelActivateAllError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.cancel_activate_all()
	}
	#[inline]
	fn migrate(&self, prcid: &itron::processor::Processor) -> Result<(), itron::error::Error<itron::task::MigrateError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.migrate(*prcid)
	}
	#[inline]
	fn get_task_state(&self) -> Result<itron::task::State, itron::error::Error<itron::task::StateError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.state()
	}
	#[inline]
	fn change_priority(&self, priority: &itron::task::Priority) -> Result<(), itron::error::Error<itron::task::SetPriorityError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.set_priority(*priority)
	}
	#[inline]
	fn change_sub_priority(&self, subPriority: &itron::abi::uint_t) -> itron::abi::ER {
		let lg = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn get_priority(&self) -> Result<itron::task::Priority, itron::error::Error<itron::task::PriorityError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.priority()
	}
	#[inline]
	fn refer(&self) -> Result<itron::task::Info, itron::error::Error<itron::task::InfoError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.info()
	}
	#[inline]
	fn wakeup(&self) -> Result<(), itron::error::Error<itron::task::WakeError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.wake()
	}
	#[inline]
	fn cancel_wakeup(&self) -> Result<usize, itron::error::Error<itron::task::CancelWakeAllError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.cancel_wake_all()
	}
	#[inline]
	fn release_wait(&self) -> Result<(), itron::error::Error<itron::task::ReleaseWaitError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.release_wait()
	}
	#[inline]
	fn suspend(&self) -> Result<(), itron::error::Error<itron::task::SuspendError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.suspend()
	}
	#[inline]
	fn resume(&self) -> Result<(), itron::error::Error<itron::task::ResumeError>> {
		let lg = self.cell.get_cell_ref();
		lg.task_ref.resume()
	}
	#[inline]
	fn raise_terminate(&self) -> Result<(), itron::error::Error<itron::task::RaiseTerminationError>> {
		let lg = self.cell.get_cell_ref();
		unsafe{
			lg.task_ref.raise_termination()
		}
	}
	#[inline]
	fn terminate(&self) -> Result<(), itron::error::Error<itron::task::TerminateError>> {
		let lg = self.cell.get_cell_ref();
		unsafe{
			lg.task_ref.terminate()
		}
	}
}

impl SiTaskRs for EiTaskForTTaskRs{

	#[inline]
	fn activate(&self) -> itron::abi::ER {
		let lg = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn wakeup(&self) -> itron::abi::ER {
		let lg = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn release_wait(&self) -> itron::abi::ER {
		let lg = self.cell.get_cell_ref();
		1
	}
}

