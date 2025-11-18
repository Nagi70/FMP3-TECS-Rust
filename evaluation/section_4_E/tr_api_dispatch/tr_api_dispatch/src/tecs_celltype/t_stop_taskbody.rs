use crate::tecs_global::*;
use crate::tecs_signature::s_task_rs::*;
use crate::tecs_celltype::t_task_rs::*;
pub struct TStopTaskbody<T>
where
	T: STaskRs + 'static,
{
	c_self_task: &'static T,
}

pub struct ETaskbodyForTStopTaskbody {
	pub cell: &'static TStopTaskbody<ETaskForTTaskRs>,
}

pub struct LockGuardForTStopTaskbody<'a, T>
where
	T: STaskRs + 'static,
{
	pub c_self_task: &'a T,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSORALLMIG_STOPTASKMIGBODY: TStopTaskbody<ETaskForTTaskRs> = TStopTaskbody {
	c_self_task: &ETASKFORRPROCESSORALLMIG_TASKMIG,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSORALLMIG_STOPTASKMIGBODY: ETaskbodyForTStopTaskbody = ETaskbodyForTStopTaskbody {
	cell: &RPROCESSORALLMIG_STOPTASKMIGBODY,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_STOPTASKBODY: TStopTaskbody<ETaskForTTaskRs> = TStopTaskbody {
	c_self_task: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_STOPTASKBODY: ETaskbodyForTStopTaskbody = ETaskbodyForTStopTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_STOPTASKBODY,
};

impl<T: STaskRs> TStopTaskbody<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTStopTaskbody<'_, T> {
		LockGuardForTStopTaskbody {
			c_self_task: self.c_self_task,
		}
	}
}
