use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::{s_task_body::*, t_x_uart_taskbody::*, t_taskbody::*};

pub struct TTaskRs<'a>{
	pub c_task_body: &'a (dyn STaskBody + Sync + Send),
	task_ref: TaskRef<'a>,
}

pub struct ETaskForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a>,
}

pub struct EiTaskForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a>,
}

pub struct EiActivateNotificationHandlerForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a>,
}

pub struct EiWakeUpNotificationHandlerForTTaskRs<'a>{
	pub cell: &'a TTaskRs<'a>,
}

pub struct LockGuardForTTaskRs<'a>{
	pub c_task_body: &'a (dyn STaskBody + Sync + Send),
	pub task_ref: &'a TaskRef<'a>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_TASK1: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_UART).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_TASK1: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_TASK1: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASK2: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_LOOP).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK2: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK2: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_TASK2: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_TASK2: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2,
};

impl<> TTaskRs<'_> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskRs<'_> {
		LockGuardForTTaskRs {
			c_task_body: self.c_task_body,
			task_ref: &self.task_ref,
		}
	}
}
