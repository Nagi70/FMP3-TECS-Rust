use crate::tecs_global::*;
pub trait SDataqueueRs {
	fn send(&self, data: &DataElement)-> Result<(), Error<SendError>>;
	fn send_polling(&self, data: &DataElement)-> Result<(), Error<TrySendError>>;
	fn send_timeout(&self, data: &DataElement, timeout: &Timeout)-> Result<(), Error<SendTimeoutError>>;
	fn send_force(&self, data: &DataElement)-> Result<(), Error<SendForcedError>>;
	fn receive(&self)-> Result<DataElement, Error<RecvError>>;
	fn receive_polling(&self)-> Result<DataElement, Error<TryRecvError>>;
	fn receive_timeout(&self, timeout: &Timeout)-> Result<DataElement, Error<RecvTimeoutError>>;
	fn initialize(&self)-> Result<(), Error<InitializeError>>;
	fn refer(&self)-> Result<Info, Error<InfoError>>;
}
