use crate::tecs_global::*;
use crate::tecs_celltype::t_dummy_can_taskbody::*;
use crate::tecs_signature::{s_task_body::*, s_can_measure_for_opt::*};
impl STaskBody for ETaskbodyForTDummyCanTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

		lg.c_can.loopback_setup();
	}
}

