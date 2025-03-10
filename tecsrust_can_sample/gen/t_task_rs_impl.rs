use crate::{t_task_rs::*, s_task_rs::*, si_task::*, s_task_body::*, si_notification_handler::*};

impl STaskRs for ETaskForTTaskRs<'_>{

	#[inline]
	fn activate(&'static self) -> Result<(), Error<ActivateError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn migrate_and_activate(&'static self, prcid: &Processor) -> Result<(), Error<ActivateOnError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn cancel_activate(&'static self) -> Result<usize, Error<CancelActivateAllError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn migrate(&'static self, prcid: &Processor) -> Result<(), Error<MigrateError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn get_task_state(&'static self) -> Result<State, Error<StateError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn change_priority(&'static self, priority: &Priority) -> Result<(), Error<SetPriorityError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn change_sub_priority(&'static self, subPriority: &uint_t) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn get_priority(&'static self) -> Result<Priority, Error<PriorityError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn refer(&'static self) -> Result<Info, Error<InfoError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn wakeup(&'static self) -> Result<(), Error<WakeError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn cancel_wakeup(&'static self) -> Result<usize, Error<CancelWakeAllError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn release_wait(&'static self) -> Result<(), Error<ReleaseWaitError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn suspend(&'static self) -> Result<(), Error<SuspendError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn resume(&'static self) -> Result<(), Error<ResumeError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn raise_terminate(&'static self) -> Result<(), Error<RaiseTerminationError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn terminate(&'static self) -> Result<(), Error<TerminateError>>{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
}

impl SiTask for EiTaskForTTaskRs<'_>{

	#[inline]
	fn activate(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn wakeup(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
	#[inline]
	fn release_wait(&'static self) -> ER{
		let (c_task_body, task_ref) = self.cell.get_cell_ref();

	}
}

impl SiNotificationHandler for EiActivateNotificationHandlerForTTaskRs<'_>{

}

impl SiNotificationHandler for EiWakeUpNotificationHandlerForTTaskRs<'_>{

}

