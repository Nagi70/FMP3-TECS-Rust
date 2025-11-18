use crate::tecs_global::*;
use crate::tecs_celltype::t_taskbody::*;
use crate::tecs_signature::s_task_body::*;
impl STaskBody for ETaskbodyForTTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

		if *lg.is_exclusive {
			// 排他制御ありタスクの処理
			lg.var.dummy += 5;
		} else {
			// 排他制御なしタスクの処理
			lg.var.dummy += 10;
		}
	}
}

