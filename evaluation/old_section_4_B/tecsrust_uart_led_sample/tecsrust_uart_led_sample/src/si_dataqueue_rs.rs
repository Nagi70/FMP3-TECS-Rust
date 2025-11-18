use itron::dataqueue::*;
use itron::error::*;

pub trait SiDataqueueRs {
	fn send_polling(&'static self, data: &DataElement)-> Result<(), Error<TrySendError>>;
	fn send_force(&'static self, data: &DataElement)-> Result<(), Error<SendForcedError>>;
}
