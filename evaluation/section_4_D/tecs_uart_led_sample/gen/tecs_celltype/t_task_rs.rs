use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::{t_x_uart_taskbody::*, t_taskbody::*};

pub trait TTaskRsConfig: 'static {
    const TASK_REF: itron::task::TaskRef<'static>;
}

pub struct TTaskRsTaskRef<CONFIG: TTaskRsConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TTaskRsConfig> core::ops::Deref for TTaskRsTaskRef<CONFIG> {
    type Target = itron::task::TaskRef<'static>;
    #[inline(always)]
    fn deref(&self) -> &itron::task::TaskRef<'static> {
        &CONFIG::TASK_REF
    }
}

// Instance Configurations
pub struct ConfigUarttask;
impl TTaskRsConfig for ConfigUarttask {
    const TASK_REF: itron::task::TaskRef<'static> = unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_UART).unwrap())};
}

pub struct ConfigButtontask;
impl TTaskRsConfig for ConfigButtontask {
    const TASK_REF: itron::task::TaskRef<'static> = unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_LOOP).unwrap())};
}

pub struct TTaskRs<CONFIG>{
	pub c_task_body: &'static (dyn STaskBody + Sync + Send),
	_phantom: core::marker::PhantomData<CONFIG>,
}

pub struct ETaskForTTaskRs<CONFIG: TTaskRsConfig> {
	pub cell: &'static TTaskRs<CONFIG>,
}

pub struct EiTaskForTTaskRs<CONFIG: TTaskRsConfig> {
	pub cell: &'static TTaskRs<CONFIG>,
}

pub struct LockGuardForTTaskRs<'a, CONFIG: TTaskRsConfig>{
	pub c_task_body: &'a (dyn STaskBody + Sync + Send),
	pub task_ref: TTaskRsTaskRef<CONFIG>,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_UARTTASK: TTaskRs<ConfigUarttask> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	_phantom: core::marker::PhantomData,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR1SYMMETRIC_UARTTASK: ETaskForTTaskRs<ConfigUarttask> = ETaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR1SYMMETRIC_UARTTASK: EiTaskForTTaskRs<ConfigUarttask> = EiTaskForTTaskRs {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASK,
};

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR2SYMMETRIC_BUTTONTASK: TTaskRs<ConfigButtontask> = TTaskRs {
	c_task_body: &ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY,
	_phantom: core::marker::PhantomData,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKFORRPROCESSOR2SYMMETRIC_BUTTONTASK: ETaskForTTaskRs<ConfigButtontask> = ETaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
};

#[unsafe(link_section = ".rodata")]
pub static EITASKFORRPROCESSOR2SYMMETRIC_BUTTONTASK: EiTaskForTTaskRs<ConfigButtontask> = EiTaskForTTaskRs {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASK,
};

impl<CONFIG: TTaskRsConfig> TTaskRs<CONFIG> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskRs<CONFIG> {
		LockGuardForTTaskRs {
			c_task_body: self.c_task_body,
			task_ref: TTaskRsTaskRef(core::marker::PhantomData),
		}
	}
}
