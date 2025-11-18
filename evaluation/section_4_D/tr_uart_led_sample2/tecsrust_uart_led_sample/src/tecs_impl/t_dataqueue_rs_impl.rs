use crate::tecs_global::*;
use crate::tecs_celltype::t_dataqueue_rs::*;
use crate::tecs_signature::{s_dataqueue_rs::*, si_dataqueue_rs::*};
use itron::dataqueue::*;
use itron::error::*;
use itron::time::*;
impl SDataqueueRs for EDataqueueForTDataqueueRs{

	#[inline]
	fn send(&self, data: &DataElement) -> Result<(), Error<SendError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send(*data)

	}
	#[inline]
	fn send_polling(&self, data: &DataElement) -> Result<(), Error<TrySendError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_send(*data)
	}
	#[inline]
	fn send_timeout(&self, data: &DataElement, timeout: &Timeout) -> Result<(), Error<SendTimeoutError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_timeout(*data, *timeout)
	}
	#[inline]
	fn send_force(&self, data: &DataElement) -> Result<(), Error<SendForcedError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_forced(*data)
	}
	#[inline]
	fn receive(&self) -> Result<DataElement, Error<RecvError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.recv()
	}
	#[inline]
	fn receive_polling(&self) -> Result<DataElement, Error<TryRecvError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_recv()
	}
	#[inline]
	fn receive_timeout(&self, timeout: &Timeout) -> Result<DataElement, Error<RecvTimeoutError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.recv_timeout(*timeout)
	}
	#[inline]
	fn initialize(&self) -> Result<(), Error<InitializeError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.initialize()
	}
	#[inline]
	fn refer(&self) -> Result<Info, Error<InfoError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.info()
	}
}

impl SiDataqueueRs for EiDataqueueForTDataqueueRs{

	#[inline]
	fn send_polling(&self, data: &DataElement) -> Result<(), Error<TrySendError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_send(*data)
	}
	#[inline]
	fn send_force(&self, data: &DataElement) -> Result<(), Error<SendForcedError>> {
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_forced(*data)
	}
}

