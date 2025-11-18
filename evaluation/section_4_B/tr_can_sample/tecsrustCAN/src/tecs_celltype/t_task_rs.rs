use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::{t_can_taskbody::*, t_loop_taskbody::*};

pub struct TTaskRs{
	pub c_task_body: &'static (dyn STaskBody + Sync + Send),
	task_ref: itron::task::TaskRef<'static>,
}

pub struct ETaskForTTaskRs {
	pub cell: &'static TTaskRs,
}

pub struct EiTaskForTTaskRs {
	pub cell: &'static TTaskRs,
}

pub struct LockGuardForTTaskRs<'a>{
	pub c_task_body: &'a (dyn STaskBody + Sync + Send),
	pub task_ref: &'a itron::task::TaskRef<'static>,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_CANTASK: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_CANTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_CAN).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_CANTASK: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_CANTASK,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_CANTASK: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_CANTASK,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_LOOPTASK: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_LOOPTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_LOOP).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_LOOPTASK: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_LOOPTASK,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_LOOPTASK: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_LOOPTASK,
};

impl TTaskRs {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskRs<'_> {
		LockGuardForTTaskRs {
			c_task_body: self.c_task_body,
			task_ref: &self.task_ref,
		}
	}
}
