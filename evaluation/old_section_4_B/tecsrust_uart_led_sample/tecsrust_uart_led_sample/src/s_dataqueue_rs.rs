use itron::dataqueue::*;
use itron::error::*;
use itron::time::*;

pub trait SDataqueueRs {
	fn send(&'static self, data: &DataElement)-> Result<(), Error<SendError>>;
	fn send_polling(&'static self, data: &DataElement)-> Result<(), Error<TrySendError>>;
	fn send_timeout(&'static self, data: &DataElement, timeout: &Timeout)-> Result<(), Error<SendTimeoutError>>;
	fn send_force(&'static self, data: &DataElement)-> Result<(), Error<SendForcedError>>;
	fn receive(&'static self)-> Result<DataElement, Error<RecvError>>;
	fn receive_polling(&'static self)-> Result<DataElement, Error<TryRecvError>>;
	fn receive_timeout(&'static self, timeout: &Timeout)-> Result<DataElement, Error<RecvTimeoutError>>;
	fn initialize(&'static self)-> Result<(), Error<InitializeError>>;
	fn refer(&'static self)-> Result<Info, Error<InfoError>>;
}
