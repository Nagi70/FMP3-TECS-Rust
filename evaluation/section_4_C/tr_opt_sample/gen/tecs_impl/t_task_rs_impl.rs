use crate::tecs_global::*;
use crate::tecs_celltype::t_task_rs::*;
use crate::tecs_signature::{s_task_rs::*, si_task_rs::*, s_task_body::*};
impl STaskRs for ETaskForTTaskRs{

	#[inline]
	fn activate(&self) -> Result<(), itron::error::Error<itron::task::ActivateError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn migrate_and_activate(&self, prcid: &itron::processor::Processor) -> Result<(), itron::error::Error<itron::task::ActivateOnError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn cancel_activate(&self) -> Result<usize, itron::error::Error<itron::task::CancelActivateAllError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn migrate(&self, prcid: &itron::processor::Processor) -> Result<(), itron::error::Error<itron::task::MigrateError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn get_task_state(&self) -> Result<itron::task::State, itron::error::Error<itron::task::StateError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn change_priority(&self, priority: &itron::task::Priority) -> Result<(), itron::error::Error<itron::task::SetPriorityError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn change_sub_priority(&self, subPriority: &itron::abi::uint_t) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn get_priority(&self) -> Result<itron::task::Priority, itron::error::Error<itron::task::PriorityError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn refer(&self) -> Result<itron::task::Info, itron::error::Error<itron::task::InfoError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn wakeup(&self) -> Result<(), itron::error::Error<itron::task::WakeError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn cancel_wakeup(&self) -> Result<usize, itron::error::Error<itron::task::CancelWakeAllError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn release_wait(&self) -> Result<(), itron::error::Error<itron::task::ReleaseWaitError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn suspend(&self) -> Result<(), itron::error::Error<itron::task::SuspendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn resume(&self) -> Result<(), itron::error::Error<itron::task::ResumeError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn raise_terminate(&self) -> Result<(), itron::error::Error<itron::task::RaiseTerminationError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn terminate(&self) -> Result<(), itron::error::Error<itron::task::TerminateError>>{
		let lg = self.cell.get_cell_ref();

	}
}

impl SiTaskRs for EiTaskForTTaskRs{

	#[inline]
	fn activate(&self) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn wakeup(&self) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn release_wait(&self) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
}

