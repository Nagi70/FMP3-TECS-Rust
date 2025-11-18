use crate::{t_x_uart_taskbody::*, s_x_uart_measure::*, s_led::*, si_dataqueue_rs::*, s_dataqueue_rs::*, s_task_body::*, si_sio_cbr::*};

impl STaskBody for ETaskbodyForTXUartTaskbody<'_>{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
}

impl SiSioCbr for EXUartMainForTXUartTaskbody<'_>{

	fn ready_send(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
	fn ready_receive(&'static self) {
		let lg = self.cell.get_cell_ref();

	}
}

