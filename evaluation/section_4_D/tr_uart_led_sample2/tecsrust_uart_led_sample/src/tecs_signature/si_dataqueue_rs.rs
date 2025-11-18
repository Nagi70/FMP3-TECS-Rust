use crate::tecs_global::*;
use itron::dataqueue::*;
use itron::error::*;
pub trait SiDataqueueRs {
	fn send_polling(&self, data: &DataElement)-> Result<(), Error<TrySendError>>;
	fn send_force(&self, data: &DataElement)-> Result<(), Error<SendForcedError>>;
}
