use crate::{s_task_rs::*, t_task_rs::*};

pub struct TStopTaskbody<'a, T>where
	T: STaskRs,
{
	c_self_task: &'a T,
	api: u32,
}

pub struct ETaskbodyForTStopTaskbody<'a>{
	pub cell: &'a TStopTaskbody<'a, ETaskForTTaskRs<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSORALLMIG_STOPTASKMIGBODY: TStopTaskbody<ETaskForTTaskRs> = TStopTaskbody {
	c_self_task: &ETASKFORRPROCESSORALLMIG_TASKMIG,
	api: 2,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSORALLMIG_STOPTASKMIGBODY: ETaskbodyForTStopTaskbody = ETaskbodyForTStopTaskbody {
	cell: &RPROCESSORALLMIG_STOPTASKMIGBODY,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_STOPTASKBODY: TStopTaskbody<ETaskForTTaskRs> = TStopTaskbody {
	c_self_task: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2,
	api: 1,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_STOPTASKBODY: ETaskbodyForTStopTaskbody = ETaskbodyForTStopTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_STOPTASKBODY,
};

impl<T: STaskRs> TStopTaskbody<'_, T> {
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static u32) {
		(
			self.c_self_task,
			&self.api
		)
	}
}
