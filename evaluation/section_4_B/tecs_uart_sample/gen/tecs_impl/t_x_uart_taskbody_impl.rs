use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
use crate::tecs_signature::{s_x_uart::*, s_dataqueue_rs::*, s_task_body::*};
impl STaskBody for ETaskbodyForTXUartTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

	}
}

