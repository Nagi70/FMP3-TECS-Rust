// #[<PREAMBLE>]#
//   Don't edit the comments between #[<...>]# and #[</...>]#
//   These comments are used by tecsmerge when merging.
//
//   call port: cXUart signature: sXUartMeasure
//   call port: cLed signature: sLed
//   call port: cDataqueue signature: siDataqueueRs
//   call port: cDataqueueLed signature: sDataqueueRs
// #[</PREAMBLE>]#

use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_led::*, si_dataqueue_rs::*, s_dataqueue_rs::*, s_task_body::*, si_sio_cbr::*};

// #[<ENTRY_PORT>]# ETaskbody
//   entry port: ETaskbody
//   signature:  STaskBody
// #[</ENTRY_PORT>]#

impl STaskBody for ETaskbodyForTXUartTaskbody {

	// #[<ENTRY_FUNC>]# ETaskbody_main
	// #[</ENTRY_FUNC>]#
	fn main(&self) {
		let lg = self.cell.get_cell_ref();

	}
}


// #[<ENTRY_PORT>]# EXUartMain
//   entry port: EXUartMain
//   signature:  SiSioCbr
// #[</ENTRY_PORT>]#

impl SiSioCbr for EXUartMainForTXUartTaskbody {

	// #[<ENTRY_FUNC>]# EXUartMain_ready_send
	// #[</ENTRY_FUNC>]#
	fn ready_send(&self) {
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EXUartMain_ready_receive
	// #[</ENTRY_FUNC>]#
	fn ready_receive(&self) {
		let lg = self.cell.get_cell_ref();

	}
}

// #[<POSTAMBLE>]#
//   Put non-entry functions below.
// #[</POSTAMBLE>]#
