use crate::tecs_global::*;
use itron::abi::*;
use itron::task::*;
use itron::error::Error;
use itron::processor::Processor;
pub trait STaskRs {
	fn activate(&self)-> Result<(), Error<ActivateError>>;
	fn migrate_and_activate(&self, prcid: &Processor)-> Result<(), Error<ActivateOnError>>;
	fn cancel_activate(&self)-> Result<usize, Error<CancelActivateAllError>>;
	fn migrate(&self, prcid: &Processor)-> Result<(), Error<MigrateError>>;
	fn get_task_state(&self)-> Result<State, Error<StateError>>;
	fn change_priority(&self, priority: &Priority)-> Result<(), Error<SetPriorityError>>;
	fn change_sub_priority(&self, subPriority: &uint_t)-> ER;
	fn get_priority(&self)-> Result<Priority, Error<PriorityError>>;
	fn refer(&self)-> Result<Info, Error<InfoError>>;
	fn wakeup(&self)-> Result<(), Error<WakeError>>;
	fn cancel_wakeup(&self)-> Result<usize, Error<CancelWakeAllError>>;
	fn release_wait(&self)-> Result<(), Error<ReleaseWaitError>>;
	fn suspend(&self)-> Result<(), Error<SuspendError>>;
	fn resume(&self)-> Result<(), Error<ResumeError>>;
	fn raise_terminate(&self)-> Result<(), Error<RaiseTerminationError>>;
	fn terminate(&self)-> Result<(), Error<TerminateError>>;
}
