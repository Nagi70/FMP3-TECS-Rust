use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::t_taskbody::*;
pub struct TTaskRs<T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'static T,
	task_ref: itron::task::TaskRef<'static>,
}

pub struct ETaskForTTaskRs {
	pub cell: &'static TTaskRs<ETaskbodyForTTaskbody>,
}

pub struct EiTaskForTTaskRs {
	pub cell: &'static TTaskRs<ETaskbodyForTTaskbody>,
}

pub struct LockGuardForTTaskRs<'a, T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'a T,
	pub task_ref: &'a itron::task::TaskRef<'static>,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_TASK1: TTaskRs<ETaskbodyForTTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY1,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TASK1_1).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_TASK2: TTaskRs<ETaskbodyForTTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY2,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TASK1_2).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK2: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK2,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK2: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK2,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_TASK3: TTaskRs<ETaskbodyForTTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY3,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TASK1_3).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK3: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK3,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK3: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK3,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_TASK4: TTaskRs<ETaskbodyForTTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY4,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TASK1_4).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK4: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK4,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK4: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK4,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_TASK1: TTaskRs<ETaskbodyForTTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY1,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TASK2_1).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK1,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK1,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_TASK2: TTaskRs<ETaskbodyForTTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY2,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TASK2_2).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK2: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK2: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_TASK3: TTaskRs<ETaskbodyForTTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY3,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TASK2_3).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK3: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK3,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK3: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK3,
};

impl<T: STaskBody> TTaskRs<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskRs<'_, T> {
		LockGuardForTTaskRs {
			c_task_body: self.c_task_body,
			task_ref: &self.task_ref,
		}
	}
}
