use crate::tecs_global::*;
pub trait SiDataqueueRs {
	fn send_polling(&self, data: &itron::dataqueue::DataElement)-> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>;
	fn send_force(&self, data: &itron::dataqueue::DataElement)-> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>;
}
