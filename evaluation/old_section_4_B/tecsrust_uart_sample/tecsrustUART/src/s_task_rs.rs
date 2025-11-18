use itron::abi::*;
use itron::task::*;
use itron::error::Error;
use itron::processor::Processor;

pub trait STaskRs {
	fn activate(&'static self)-> Result<(), Error<ActivateError>>;
	fn migrate_and_activate(&'static self, prcid: &Processor)-> Result<(), Error<ActivateOnError>>;
	fn cancel_activate(&'static self)-> Result<usize, Error<CancelActivateAllError>>;
	fn migrate(&'static self, prcid: &Processor)-> Result<(), Error<MigrateError>>;
	fn get_task_state(&'static self)-> Result<State, Error<StateError>>;
	fn change_priority(&'static self, priority: &Priority)-> Result<(), Error<SetPriorityError>>;
	fn change_sub_priority(&'static self, subPriority: &uint_t)-> ER;
	fn get_priority(&'static self)-> Result<Priority, Error<PriorityError>>;
	fn refer(&'static self)-> Result<Info, Error<InfoError>>;
	fn wakeup(&'static self)-> Result<(), Error<WakeError>>;
	fn cancel_wakeup(&'static self)-> Result<usize, Error<CancelWakeAllError>>;
	fn release_wait(&'static self)-> Result<(), Error<ReleaseWaitError>>;
	fn suspend(&'static self)-> Result<(), Error<SuspendError>>;
	fn resume(&'static self)-> Result<(), Error<ResumeError>>;
	fn raise_terminate(&'static self)-> Result<(), Error<RaiseTerminationError>>;
	fn terminate(&'static self)-> Result<(), Error<TerminateError>>;
}
