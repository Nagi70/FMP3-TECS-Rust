use crate::tecs_global::*;
use crate::tecs_celltype::t_led_taskbody::*;
use crate::tecs_signature::{s_led::*, s_task_body::*};
impl STaskBody for ETaskbodyForTLedTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

	}
}

