use crate::{t_reset::*, s_task_rs::*, s_semaphore_rs::*, s_task_body::*};

impl STaskBody for EResetForTReset<'_>{

	fn main(&'static self) {
		let (c_task, c_taskself, c_taskmig, c_semaphore) = self.cell.get_cell_ref();

	}
}

