// #[<PREAMBLE>]#
//   Don't edit the comments between #[<...>]# and #[</...>]#
//   These comments are used by tecsmerge when merging.
//
// #[</PREAMBLE>]#

use crate::tecs_global::*;
use crate::tecs_celltype::t_dataqueue_rs::*;
use crate::tecs_signature::{s_dataqueue_rs::*, si_dataqueue_rs::*};

// #[<ENTRY_PORT>]# EDataqueue
//   entry port: EDataqueue
//   signature:  SDataqueueRs
// #[</ENTRY_PORT>]#

impl<CONFIG: TDataqueueRsConfig> SDataqueueRs for EDataqueueForTDataqueueRs<CONFIG> {

	// #[<ENTRY_FUNC>]# EDataqueue_send
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn send(&self, data: itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_send_polling
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn send_polling(&self, data: itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_send_timeout
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn send_timeout(&self, data: itron::dataqueue::DataElement, timeout: itron::time::Timeout) -> Result<(), itron::error::Error<itron::dataqueue::SendTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_send_force
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn send_force(&self, data: itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_receive
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn receive(&self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_receive_polling
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn receive_polling(&self) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::TryRecvError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_receive_timeout
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn receive_timeout(&self, timeout: itron::time::Timeout) -> Result<itron::dataqueue::DataElement, itron::error::Error<itron::dataqueue::RecvTimeoutError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_initialize
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn initialize(&self) -> Result<(), itron::error::Error<itron::dataqueue::InitializeError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EDataqueue_refer
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn refer(&self) -> Result<itron::dataqueue::Info, itron::error::Error<itron::dataqueue::InfoError>>{
		let lg = self.cell.get_cell_ref();

	}
}


// #[<ENTRY_PORT>]# EiDataqueue
//   entry port: EiDataqueue
//   signature:  SiDataqueueRs
// #[</ENTRY_PORT>]#

impl<CONFIG: TDataqueueRsConfig> SiDataqueueRs for EiDataqueueForTDataqueueRs<CONFIG> {

	// #[<ENTRY_FUNC>]# EiDataqueue_send_polling
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn send_polling(&self, data: itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::TrySendError>>{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EiDataqueue_send_force
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn send_force(&self, data: itron::dataqueue::DataElement) -> Result<(), itron::error::Error<itron::dataqueue::SendForcedError>>{
		let lg = self.cell.get_cell_ref();

	}
}

// #[<POSTAMBLE>]#
//   Put non-entry functions below.
// #[</POSTAMBLE>]#
