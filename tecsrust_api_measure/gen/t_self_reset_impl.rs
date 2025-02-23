use crate::{t_self_reset::*, s_task_rs::*, s_task_body::*, s_semaphore_rs::*};

impl STaskBody for ESelfResetForTSelfReset<'_>{

	fn main(&'static self) {
		let (c_taskself, c_semaphore) = self.cell.get_cell_ref();

	}
}

