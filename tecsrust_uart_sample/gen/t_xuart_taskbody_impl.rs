use crate::{t_xuart_taskbody::*, s_xuart_measure::*, s_task_body::*, si_sio_cbr::*};

impl STaskBody for ETaskbodyForTXuartTaskbody<'_>{

	fn main(&'static self) {
		let c_xuart = self.cell.get_cell_ref();

	}
}

impl SiSioCbr for EXuartMainForTXuartTaskbody<'_>{

	fn ready_send(&'static self) {
		let c_xuart = self.cell.get_cell_ref();

	}
	fn ready_receive(&'static self) {
		let c_xuart = self.cell.get_cell_ref();

	}
}

