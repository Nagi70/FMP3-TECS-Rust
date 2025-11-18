use crate::{t_measure::*, s_task_rs::*, s_semaphore_rs::*, s_task_body::*};

impl STaskBody for ETaskbodyForTMeasure<'_>{

	fn main(&'static self) {
		let (c_task, c_taskmig, c_semaphore) = self.cell.get_cell_ref();

	}
}

