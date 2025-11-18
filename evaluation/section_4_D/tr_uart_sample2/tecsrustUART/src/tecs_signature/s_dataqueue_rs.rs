use crate::tecs_global::*;
use itron::dataqueue::*;
use itron::error::*;
use itron::time::*;
pub trait SDataqueueRs {
	fn send(&self, data: &itron::dataqueue::DataElement)-> Result<(), itron::error::Error<itron::dataqueue::SendError>>;
	fn send_polling(&self, data: &itron::dataqueue::DataElement)-> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>;
	fn send_timeout(&self, data: &itron::dataqueue::DataElement, timeout: &itron::time::Timeout)-> Result<(), itron::error::Error<itron::dataqueue::SendTimeoutError>>;
	fn send_force(&self, data: &itron::dataqueue::DataElement)-> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>;
	fn receive(&self)-> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvError>>;
	fn receive_polling(&self)-> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::TryRecvError>>;
	fn receive_timeout(&self, timeout: &itron::time::Timeout)-> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvTimeoutError>>;
	fn initialize(&self)-> Result<(), itron::error::Error<itron::dataqueue::InitializeError>>;
	fn refer(&self)-> Result<itron::dataqueue::Info, itron::error::Error<itron::dataqueue::InfoError>>;
}
