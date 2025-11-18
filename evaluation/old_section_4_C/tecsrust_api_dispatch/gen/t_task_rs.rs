use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::{s_task_body::*, t_measure::*, t_stop_taskbody::*, t_loop_taskbody::*};

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

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_TASK1_1: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_MEASURE,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_1_1).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR1SYMMETRIC_TASK1_1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1_1,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR1SYMMETRIC_TASK1_1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1_1,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_TASK1_1: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1_1,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR1SYMMETRIC_TASK1_1: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_TASK1_1,
};

#[link_section = ".rodata"]
pub static RPROCESSORALLMIG_TASKMIG: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSORALLMIG_STOPTASKMIGBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_MIG).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSORALLMIG_TASKMIG: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSORALLMIG_TASKMIG,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSORALLMIG_TASKMIG: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSORALLMIG_TASKMIG,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSORALLMIG_TASKMIG: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSORALLMIG_TASKMIG,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSORALLMIG_TASKMIG: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSORALLMIG_TASKMIG,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASK2_1: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_LOOPTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_2_1).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK2_1: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_1,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK2_1: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_1,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_TASK2_1: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_1,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_TASK2_1: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_1,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASK2_2: TTaskRs = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_STOPTASKBODY,
	task_ref: unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_2_2).unwrap())},
};

#[link_section = ".rodata"]
pub static ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2: ETaskForTTaskRs = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_2,
};

#[link_section = ".rodata"]
pub static EITASKFORRPROCESSOR2SYMMETRIC_TASK2_2: EiTaskForTTaskRs = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_2,
};

#[link_section = ".rodata"]
pub static EIACTIVATENOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_TASK2_2: EiActivateNotificationHandlerForTTaskRs = EiActivateNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_2,
};

#[link_section = ".rodata"]
pub static EIWAKEUPNOTIFICATIONHANDLERFORRPROCESSOR2SYMMETRIC_TASK2_2: EiWakeUpNotificationHandlerForTTaskRs = EiWakeUpNotificationHandlerForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_TASK2_2,
};

impl<> TTaskRs<'_> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> (&'static dyn STaskBody, &'static TaskRef) {
		(
			self.c_task_body,
			&self.task_ref
		)
	}
}
