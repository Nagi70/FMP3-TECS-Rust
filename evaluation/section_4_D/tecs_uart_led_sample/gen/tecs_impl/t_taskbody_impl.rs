// #[<PREAMBLE>]#
//   Don't edit the comments between #[<...>]# and #[</...>]#
//   These comments are used by tecsmerge when merging.
//
//   call port: cXUart signature: sXUartMeasure
//   call port: cDataqueue signature: sDataqueueRs
//   call port: cDataqueueLed signature: sDataqueueRs
// #[</PREAMBLE>]#

use crate::tecs_global::*;
use crate::tecs_celltype::t_taskbody::*;
use crate::tecs_signature::{s_task_body::*, s_x_uart_measure::*, s_dataqueue_rs::*};

// #[<ENTRY_PORT>]# ETaskbody
//   entry port: ETaskbody
//   signature:  STaskBody
// #[</ENTRY_PORT>]#

impl STaskBody for ETaskbodyForTTaskbody {

	// #[<ENTRY_FUNC>]# ETaskbody_main
	// #[</ENTRY_FUNC>]#
	fn main(&self) {
		let lg = self.cell.get_cell_ref();

	}
}

// #[<POSTAMBLE>]#
//   Put non-entry functions below.
// #[</POSTAMBLE>]#
