pub trait SSemaphoreRs {
	fn signal(&'static self)-> Result<(), Error<SignalError>>;
	fn wait(&'static self)-> Result<(), Error<WaitError>>;
	fn wait_polling(&'static self)-> Result<(), Error<PollError>>;
	fn wait_timeout(&'static self, timeout: &Timeout)-> Result<(), Error<WaitTimeoutError>>;
	fn initialize(&'static self)-> Result<(), Error<InitializeError>>;
	fn refer(&'static self)-> Result<Info, Error<InfoError>>;
}
