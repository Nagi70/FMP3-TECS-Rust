use crate::tecs_global::*;
pub trait SSemaphoreRs {
	fn signal(&self)-> Result<(), itron::error::Error<itron::semaphore::SignalError>>;
	fn wait(&self)-> Result<(), itron::error::Error<itron::semaphore::WaitError>>;
	fn wait_polling(&self)-> Result<(), itron::error::Error<itron::semaphore::PollError>>;
	fn wait_timeout(&self, timeout: &itron::time::Timeout)-> Result<(), itron::error::Error<itron::semaphore::WaitTimeoutError>>;
	fn initialize(&self)-> Result<(), itron::error::Error<itron::semaphore::InitializeError>>;
	fn refer(&self)-> Result<itron::semaphore::Info, itron::error::Error<itron::semaphore::InfoError>>;
}
