use crate::{t_task_rs::*, s_task_rs::*, si_task::*, s_task_body::*, si_notification_handler::*};
use itron::abi::*;
use itron::task::*;
use itron::error::Error;
use itron::processor::Processor;


impl STaskRs for ETaskForTTaskRs<'_>{

	#[inline]
	fn activate(&'static self) -> Result<(), Error<ActivateError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.activate()
	}
	#[inline]
	fn migrate_and_activate(&'static self, prcid: &Processor) -> Result<(), Error<ActivateOnError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.activate_on(*prcid)
	}
	#[inline]
	fn cancel_activate(&'static self) -> Result<usize, Error<CancelActivateAllError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.cancel_activate_all()
	}
	#[inline]
	fn migrate(&'static self, prcid: &Processor) -> Result<(), Error<MigrateError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.migrate(*prcid)
	}
	#[inline]
	fn get_task_state(&'static self) -> Result<State, Error<StateError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.state()
	}
	#[inline]
	fn change_priority(&'static self, priority: &Priority) -> Result<(), Error<SetPriorityError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.set_priority(*priority)
	}
	#[inline]
	fn change_sub_priority(&'static self, subPriority: &uint_t) -> ER{
		let lg = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn get_priority(&'static self) -> Result<Priority, Error<PriorityError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.priority()
	}
	#[inline]
	fn refer(&'static self) -> Result<Info, Error<InfoError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.info()
	}
	#[inline]
	fn wakeup(&'static self) -> Result<(), Error<WakeError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.wake()
	}
	#[inline]
	fn cancel_wakeup(&'static self) -> Result<usize, Error<CancelWakeAllError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.cancel_wake_all()
	}
	#[inline]
	fn release_wait(&'static self) -> Result<(), Error<ReleaseWaitError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.release_wait()
	}
	#[inline]
	fn suspend(&'static self) -> Result<(), Error<SuspendError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.suspend()
	}
	#[inline]
	fn resume(&'static self) -> Result<(), Error<ResumeError>>{
		let lg = self.cell.get_cell_ref();
		lg.task_ref.resume()
	}
	#[inline]
	fn raise_terminate(&'static self) -> Result<(), Error<RaiseTerminationError>>{
		let lg = self.cell.get_cell_ref();
		unsafe{
			lg.task_ref.raise_termination()
		}
	}
	#[inline]
	fn terminate(&'static self) -> Result<(), Error<TerminateError>>{
		let lg = self.cell.get_cell_ref();
		unsafe{
			lg.task_ref.terminate()
		}
	}
}

impl SiTask for EiTaskForTTaskRs<'_>{

	#[inline]
	fn activate(&'static self) -> ER{
		let lg = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn wakeup(&'static self) -> ER{
		let lg = self.cell.get_cell_ref();
		1
	}
	#[inline]
	fn release_wait(&'static self) -> ER{
		let lg = self.cell.get_cell_ref();
		1
	}
}

impl SiNotificationHandler for EiActivateNotificationHandlerForTTaskRs<'_>{

}

impl SiNotificationHandler for EiWakeUpNotificationHandlerForTTaskRs<'_>{

}

