use crate::{t_stop_taskbody::*, s_task_rs::*, s_task_body::*};

impl STaskBody for ETaskbodyForTStopTaskbody<'_>{

	fn main(&'static self) {
		let (c_self_task, api) = self.cell.get_cell_ref();

	}
}

