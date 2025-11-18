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
pub static RPROCESSOR1SYMMETRIC_UARTTASK: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_UART).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR1SYMMETRIC_UARTTASK: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR1SYMMETRIC_UARTTASK: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_UARTTASK: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_UARTTASK: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_BUTTONTASK: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_LOOP).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR2SYMMETRIC_BUTTONTASK: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR2SYMMETRIC_BUTTONTASK: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_BUTTONTASK: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_BUTTONTASK: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
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
