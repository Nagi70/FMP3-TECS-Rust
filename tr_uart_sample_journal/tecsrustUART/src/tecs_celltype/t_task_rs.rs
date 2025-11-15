use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
pub struct TTaskRs<T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'static T,
	task_ref: itron::task::TaskRef<'static>,
}

pub struct ETaskForTTaskRs {
	pub cell: &'static TTaskRs<ETaskbodyForTXUartTaskbody>,
}

pub struct EiTaskForTTaskRs {
	pub cell: &'static TTaskRs<ETaskbodyForTXUartTaskbody>,
}

pub struct LockGuardForTTaskRs<'a, T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'a T,
	pub task_ref: &'a itron::task::TaskRef<'static>,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_TASK1: TTaskRs<ETaskbodyForTXUartTaskbody> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_UART).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
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
