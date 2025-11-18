use crate::tecs_global::*;
pub trait SiDataqueueRs {
	fn send_polling(&self, data: &DataElement)-> Result<(), Error<TrySendError>>;
	fn send_force(&self, data: &DataElement)-> Result<(), Error<SendForcedError>>;
}
