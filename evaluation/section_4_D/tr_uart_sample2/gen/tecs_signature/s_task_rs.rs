use crate::tecs_global::*;
pub trait STaskRs {
	fn activate(&self)-> Result<(), itron::error::Error<itron::task::ActivateError>>;
	fn migrate_and_activate(&self, prcid: &itron::processor::Processor)-> Result<(), itron::error::Error<itron::task::ActivateOnError>>;
	fn cancel_activate(&self)-> Result<usize, itron::error::Error<itron::task::CancelActivateAllError>>;
	fn migrate(&self, prcid: &itron::processor::Processor)-> Result<(), itron::error::Error<itron::task::MigrateError>>;
	fn get_task_state(&self)-> Result<itron::task::State, itron::error::Error<itron::task::StateError>>;
	fn change_priority(&self, priority: &itron::task::Priority)-> Result<(), itron::error::Error<itron::task::SetPriorityError>>;
	fn change_sub_priority(&self, subPriority: &itron::abi::uint_t)-> itron::abi::ER;
	fn get_priority(&self)-> Result<itron::task::Priority, itron::error::Error<itron::task::PriorityError>>;
	fn refer(&self)-> Result<itron::task::Info, itron::error::Error<itron::task::InfoError>>;
	fn wakeup(&self)-> Result<(), itron::error::Error<itron::task::WakeError>>;
	fn cancel_wakeup(&self)-> Result<usize, itron::error::Error<itron::task::CancelWakeAllError>>;
	fn release_wait(&self)-> Result<(), itron::error::Error<itron::task::ReleaseWaitError>>;
	fn suspend(&self)-> Result<(), itron::error::Error<itron::task::SuspendError>>;
	fn resume(&self)-> Result<(), itron::error::Error<itron::task::ResumeError>>;
	fn raise_terminate(&self)-> Result<(), itron::error::Error<itron::task::RaiseTerminationError>>;
	fn terminate(&self)-> Result<(), itron::error::Error<itron::task::TerminateError>>;
}
