use crate::tecs_global::*;
use crate::tecs_signature::s_task_rs::*;
use crate::tecs_celltype::t_task_rs::*;
pub struct TLoopTaskbody<T>
where
	T: STaskRs + 'static,
{
	c_task: &'static T,
}

pub struct ETaskbodyForTLoopTaskbody {
	pub cell: &'static TLoopTaskbody<ETaskForTTaskRs>,
}

pub struct LockGuardForTLoopTaskbody<'a, T>
where
	T: STaskRs + 'static,
{
	pub c_task: &'a T,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_LOOPTASKBODY: TLoopTaskbody<ETaskForTTaskRs> = TLoopTaskbody {
	c_task: &ETASKFORRPROCESSOR1SYMMETRIC_CANTASK,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_LOOPTASKBODY: ETaskbodyForTLoopTaskbody = ETaskbodyForTLoopTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_LOOPTASKBODY,
};

impl<T: STaskRs> TLoopTaskbody<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTLoopTaskbody<'_, T> {
		LockGuardForTLoopTaskbody {
			c_task: self.c_task,
		}
	}
}
