// #[<PREAMBLE>]#
//   Don't edit the comments between #[<...>]# and #[</...>]#
//   These comments are used by tecsmerge when merging.
//
//   call port: cTaskBody signature: sTaskBody
// #[</PREAMBLE>]#

use crate::tecs_global::*;
use crate::tecs_celltype::t_task_rs::*;
use crate::tecs_signature::{s_task_rs::*, si_task_rs::*, s_task_body::*};

// #[<ENTRY_PORT>]# ETask
//   entry port: ETask
//   signature:  STaskRs
// #[</ENTRY_PORT>]#

impl<CONFIG: TTaskRsConfig> STaskRs for ETaskForTTaskRs<CONFIG> {

	// #[<ENTRY_FUNC>]# ETask_activate
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn activate(&self) -> Result<(), itron::error::Error<itron::task::ActivateError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_migrate_and_activate
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn migrate_and_activate(&self, prcid: itron::processor::Processor) -> Result<(), itron::error::Error<itron::task::ActivateOnError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_cancel_activate
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn cancel_activate(&self) -> Result<usize, itron::error::Error<itron::task::CancelActivateAllError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_migrate
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn migrate(&self, prcid: itron::processor::Processor) -> Result<(), itron::error::Error<itron::task::MigrateError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_get_task_state
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn get_task_state(&self) -> Result<itron::task::State, itron::error::Error<itron::task::StateError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_change_priority
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn change_priority(&self, priority: itron::task::Priority) -> Result<(), itron::error::Error<itron::task::SetPriorityError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_change_sub_priority
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn change_sub_priority(&self, subPriority: itron::abi::uint_t) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_get_priority
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn get_priority(&self) -> Result<itron::task::Priority, itron::error::Error<itron::task::PriorityError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_refer
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn refer(&self) -> Result<itron::task::Info, itron::error::Error<itron::task::InfoError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_wakeup
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn wakeup(&self) -> Result<(), itron::error::Error<itron::task::WakeError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_cancel_wakeup
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn cancel_wakeup(&self) -> Result<usize, itron::error::Error<itron::task::CancelWakeAllError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_release_wait
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn release_wait(&self) -> Result<(), itron::error::Error<itron::task::ReleaseWaitError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_suspend
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn suspend(&self) -> Result<(), itron::error::Error<itron::task::SuspendError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_resume
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn resume(&self) -> Result<(), itron::error::Error<itron::task::ResumeError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_raise_terminate
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn raise_terminate(&self) -> Result<(), itron::error::Error<itron::task::RaiseTerminationError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ETask_terminate
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn terminate(&self) -> Result<(), itron::error::Error<itron::task::TerminateError>>{
		let lg = self.cell.get_cell_ref();

	}
}


// #[<ENTRY_PORT>]# EiTask
//   entry port: EiTask
//   signature:  SiTaskRs
// #[</ENTRY_PORT>]#

impl<CONFIG: TTaskRsConfig> SiTaskRs for EiTaskForTTaskRs<CONFIG> {

	// #[<ENTRY_FUNC>]# EiTask_activate
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn activate(&self) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EiTask_wakeup
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn wakeup(&self) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EiTask_release_wait
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn release_wait(&self) -> itron::abi::ER{
		let lg = self.cell.get_cell_ref();

	}
}

// #[<POSTAMBLE>]#
//   Put non-entry functions below.
// #[</POSTAMBLE>]#
