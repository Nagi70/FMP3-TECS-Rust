use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::{t_x_uart_taskbody::*, t_taskbody::*};

pub struct TTaskRs{
	pub c_task_body: &'static (dyn STaskBody + Sync + Send),
	task_ref: TaskRef<'static>,
}

pub struct ETaskForTTaskRs {
	pub cell: &'static TTaskRs,
}

pub struct EiTaskForTTaskRs {
	pub cell: &'static TTaskRs,
}

pub struct LockGuardForTTaskRs<'a>{
	pub c_task_body: &'a (dyn STaskBody + Sync + Send),
	pub task_ref: &'a TaskRef<'static>,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_UARTTASK: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_UART).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_UARTTASK: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_UARTTASK: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_BUTTONTASK: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_LOOP).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_BUTTONTASK: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_BUTTONTASK: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
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
