use crate::{t_loop_taskbody::*, s_task_rs::*, s_task_body::*};

impl STaskBody for ETaskbodyForTLoopTaskbody<'_>{

	fn main(&'static self) {
		let c_task = self.cell.get_cell_ref();

	}
}

