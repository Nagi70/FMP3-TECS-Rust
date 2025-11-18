use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::{t_measure::*, t_stop_taskbody::*, t_loop_taskbody::*};

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
pub static RPROCESSOR1SYMMETRIC_TASK1_1: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_MEASURE,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_1_1).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK1_1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1_1,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK1_1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1_1,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSORALLMIG_TASKMIG: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSORALLMIG_STOPTASKMIGBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_MIG).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSORALLMIG_TASKMIG: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSORALLMIG_TASKMIG,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSORALLMIG_TASKMIG: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSORALLMIG_TASKMIG,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_TASK2_1: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_LOOPTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_2_1).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK2_1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_1,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK2_1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_1,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_TASK2_2: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_STOPTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_2_2).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_2,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK2_2: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_2,
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
