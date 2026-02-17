// #[<PREAMBLE>]#
//   Don't edit the comments between #[<...>]# and #[</...>]#
//   These comments are used by tecsmerge when merging.
//
//   call port: cXUartMain signature: siSioCbr
// #[</PREAMBLE>]#

use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart::*;
use crate::tecs_signature::{si_sio_cbr::*, s_x_uart_measure::*, si_handler_body::*};

// #[<ENTRY_PORT>]# EXUart
//   entry port: EXUart
//   signature:  SXUartMeasure
// #[</ENTRY_PORT>]#

impl<CONFIG: TXUartConfig> SXUartMeasure for EXUartForTXUart<CONFIG> {

	// #[<ENTRY_FUNC>]# EXUart_open
	// #[</ENTRY_FUNC>]#
	fn open(&self) {
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EXUart_put_char
	// #[</ENTRY_FUNC>]#
	fn put_char(&self, c: u8) -> bool{
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# EXUart_get_char
	// #[</ENTRY_FUNC>]#
	fn get_char(&self, c: &mut u8) -> bool{
		let lg = self.cell.get_cell_ref();

	}
}


// #[<ENTRY_PORT>]# EiHandlerBody
//   entry port: EiHandlerBody
//   signature:  SiHandlerBody
// #[</ENTRY_PORT>]#

impl<CONFIG: TXUartConfig> SiHandlerBody for EiHandlerBodyForTXUart<CONFIG> {

	// #[<ENTRY_FUNC>]# EiHandlerBody_main
	// #[</ENTRY_FUNC>]#
	fn main(&self) {
		let lg = self.cell.get_cell_ref();

	}
}

// #[<POSTAMBLE>]#
//   Put non-entry functions below.
// #[</POSTAMBLE>]#
