use crate::{s_task_rs::*, t_task_rs::*};

pub struct TLoopTaskbody<'a, T>where
	T: STaskRs,
{
	c_task: &'a T,
}

pub struct ETaskbodyForTLoopTaskbody<'a>{
	pub cell: &'a TLoopTaskbody<'a, ETaskForTTaskRs<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_LOOPTASKBODY: TLoopTaskbody<ETaskForTTaskRs> = TLoopTaskbody {
	c_task: &ETASKFORRPROCESSOR1SYMMETRIC_CANTASK,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_LOOPTASKBODY: ETaskbodyForTLoopTaskbody = ETaskbodyForTLoopTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_LOOPTASKBODY,
};

impl<T: STaskRs> TLoopTaskbody<'_, T> {
	pub fn get_cell_ref(&'static self) -> &'static T {
		self.c_task
	}
}
