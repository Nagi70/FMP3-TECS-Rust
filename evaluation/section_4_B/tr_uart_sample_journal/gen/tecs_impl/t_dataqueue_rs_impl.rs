use crate::tecs_global::*;
use crate::tecs_celltype::t_dataqueue_rs::*;
use crate::tecs_signature::{s_dataqueue_rs::*, si_dataqueue_rs::*};
impl SDataqueueRs for EDataqueueForTDataqueueRs{

	#[inline]
	fn send(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_polling(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_timeout(&self, data: &itron::dataqueue::DataElement, timeout: &itron::time::Timeout) -> Result<(), itron::error::Error<itron::dataqueue::SendTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_force(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive(&self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive_polling(&self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::TryRecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive_timeout(&self, timeout: &itron::time::Timeout) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn initialize(&self) -> Result<(), itron::error::Error<itron::dataqueue::InitializeError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn refer(&self) -> Result<itron::dataqueue::Info, itron::error::Error<itron::dataqueue::InfoError>>{
		let lg = self.cell.get_cell_ref();

	}
}

impl SiDataqueueRs for EiDataqueueForTDataqueueRs{

	#[inline]
	fn send_polling(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_force(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
}

