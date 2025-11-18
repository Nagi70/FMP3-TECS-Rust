use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_led::*, si_dataqueue_rs::*, s_dataqueue_rs::*, s_task_body::*, si_sio_cbr::*};
impl STaskBody for ETaskbodyForTXUartTaskbody{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
}

impl SiSioCbr for EXUartMainForTXUartTaskbody{

	fn ready_send(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
	fn ready_receive(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
}

