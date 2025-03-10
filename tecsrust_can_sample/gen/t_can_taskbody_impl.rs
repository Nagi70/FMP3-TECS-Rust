use crate::{t_can_taskbody::*, s_can_measure::*, s_task_body::*};

impl STaskBody for ETaskbodyForTCanTaskbody<'_>{

	fn main(&'static self) {
		let c_can = self.cell.get_cell_ref();

	}
}

