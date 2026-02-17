// #[<PREAMBLE>]#
//   Don't edit the comments between #[<...>]# and #[</...>]#
//   These comments are used by tecsmerge when merging.
//
// #[</PREAMBLE>]#

use crate::tecs_global::*;
use crate::tecs_celltype::t_mio_led::*;
use crate::tecs_signature::s_led::*;

// #[<ENTRY_PORT>]# ELed
//   entry port: ELed
//   signature:  SLed
// #[</ENTRY_PORT>]#

impl<CONFIG: TMioLedConfig> SLed for ELedForTMioLed<CONFIG> {

	// #[<ENTRY_FUNC>]# ELed_set_up
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn set_up(&self) {
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ELed_light_on
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn light_on(&self) {
		let lg = self.cell.get_cell_ref();

	}
	// #[<ENTRY_FUNC>]# ELed_light_off
	// #[</ENTRY_FUNC>]#
	#[inline]
	fn light_off(&self) {
		let lg = self.cell.get_cell_ref();

	}
}

// #[<POSTAMBLE>]#
//   Put non-entry functions below.
// #[</POSTAMBLE>]#
