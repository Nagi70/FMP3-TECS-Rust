use crate::{t_dataqueue_rs::*, s_dataqueue_rs::*, si_dataqueue_rs::*, si_notification_handler::*};
use itron::dataqueue::*;
use itron::error::*;
use itron::time::*;
impl SDataqueueRs for EDataqueueForTDataqueueRs<'_>{

	#[inline]
	fn send(&'static self, data: &DataElement) -> Result<(), Error<SendError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send(*data)

	}
	#[inline]
	fn send_polling(&'static self, data: &DataElement) -> Result<(), Error<TrySendError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_send(*data)
	}
	#[inline]
	fn send_timeout(&'static self, data: &DataElement, timeout: &Timeout) -> Result<(), Error<SendTimeoutError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_timeout(*data, *timeout)
	}
	#[inline]
	fn send_force(&'static self, data: &DataElement) -> Result<(), Error<SendForcedError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_forced(*data)
	}
	#[inline]
	fn receive(&'static self) -> Result<DataElement, Error<RecvError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.recv()
	}
	#[inline]
	fn receive_polling(&'static self) -> Result<DataElement, Error<TryRecvError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_recv()
	}
	#[inline]
	fn receive_timeout(&'static self, timeout: &Timeout) -> Result<DataElement, Error<RecvTimeoutError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.recv_timeout(*timeout)
	}
	#[inline]
	fn initialize(&'static self) -> Result<(), Error<InitializeError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.initialize()
	}
	#[inline]
	fn refer(&'static self) -> Result<Info, Error<InfoError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.info()
	}
}

impl SiDataqueueRs for EiDataqueueForTDataqueueRs<'_>{

	#[inline]
	fn send_polling(&'static self, data: &DataElement) -> Result<(), Error<TrySendError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.try_send(*data)
	}
	#[inline]
	fn send_force(&'static self, data: &DataElement) -> Result<(), Error<SendForcedError>>{
		let lg = self.cell.get_cell_ref();
		lg.dataqueue_ref.send_forced(*data)
	}
}

impl SiNotificationHandler for EiNotificationHandlerForTDataqueueRs<'_>{

}

