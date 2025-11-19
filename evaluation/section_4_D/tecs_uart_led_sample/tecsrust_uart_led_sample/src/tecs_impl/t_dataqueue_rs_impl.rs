use crate::tecs_global::*;
use crate::tecs_celltype::t_dataqueue_rs::*;
use crate::tecs_signature::{s_dataqueue_rs::*, si_dataqueue_rs::*};
use itron::dataqueue::*;
use itron::error::*;
use itron::time::*;
impl SDataqueueRs for EDataqueueForTDataqueueRs{

	#[inline]
	fn send(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send(*data)

	}
	#[inline]
	fn send_polling(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_send(*data)
	}
	#[inline]
	fn send_timeout(&self, data: &itron::dataqueue::DataElement, timeout: &itron::time::Timeout) -> Result<(), itron::error::Error<itron::dataqueue::SendTimeoutError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_timeout(*data, *timeout)
	}
	#[inline]
	fn send_force(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_forced(*data)
	}
	#[inline]
	fn receive(&self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.recv()
	}
	#[inline]
	fn receive_polling(&self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::TryRecvError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_recv()
	}
	#[inline]
	fn receive_timeout(&self, timeout: &itron::time::Timeout) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvTimeoutError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.recv_timeout(*timeout)
	}
	#[inline]
	fn initialize(&self) -> Result<(), itron::error::Error<itron::dataqueue::InitializeError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.initialize()
	}
	#[inline]
	fn refer(&self) -> Result<itron::dataqueue::Info, itron::error::Error<itron::dataqueue::InfoError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.info()
	}
}

impl SiDataqueueRs for EiDataqueueForTDataqueueRs{

	#[inline]
	fn send_polling(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_send(*data)
	}
	#[inline]
	fn send_force(&self, data: &itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_forced(*data)
	}
}

