use crate::{t_dataqueue_rs::*, s_dataqueue_rs::*, si_dataqueue_rs::*, si_notification_handler::*};

impl SDataqueueRs for EDataqueueForTDataqueueRs<'_>{

	#[inline]
	fn send(&'static self, data: &DataElement) -> Result<(), Error<SendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_polling(&'static self, data: &DataElement) -> Result<(), Error<TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_timeout(&'static self, data: &DataElement, timeout: &Timeout) -> Result<(), Error<SendTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_force(&'static self, data: &DataElement) -> Result<(), Error<SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive(&'static self) -> Result<DataElement, Error<RecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive_polling(&'static self) -> Result<DataElement, Error<TryRecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn receive_timeout(&'static self, timeout: &Timeout) -> Result<DataElement, Error<RecvTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn initialize(&'static self) -> Result<(), Error<InitializeError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn refer(&'static self) -> Result<Info, Error<InfoError>>{
		let lg = self.cell.get_cell_ref();

	}
}

impl SiDataqueueRs for EiDataqueueForTDataqueueRs<'_>{

	#[inline]
	fn send_polling(&'static self, data: &DataElement) -> Result<(), Error<TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	#[inline]
	fn send_force(&'static self, data: &DataElement) -> Result<(), Error<SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
}

impl SiNotificationHandler for EiNotificationHandlerForTDataqueueRs<'_>{

}

