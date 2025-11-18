use crate::tecs_global::*;
use crate::tecs_signature::s_task_rs::*;
use crate::tecs_celltype::t_task_rs::*;
pub struct TMeasure<T, U>
where
	T: STaskRs + 'static,
	U: STaskRs + 'static,
{
	c_task: &'static T,
	c_taskmig: &'static U,
}

pub struct ETaskbodyForTMeasure {
	pub cell: &'static TMeasure<ETaskForTTaskRs, ETaskForTTaskRs>,
}

pub struct LockGuardForTMeasure<'a, T, U>
where
	T: STaskRs + 'static,
	U: STaskRs + 'static,
{
	pub c_task: &'a T,
	pub c_taskmig: &'a U,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_MEASURE: TMeasure<ETaskForTTaskRs, ETaskForTTaskRs> = TMeasure {
	c_task: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2,
	c_taskmig: &ETASKFORRPROCESSORALLMIG_TASKMIG,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_MEASURE: ETaskbodyForTMeasure = ETaskbodyForTMeasure {
	cell: &RPROCESSOR1SYMMETRIC_MEASURE,
};

impl<T: STaskRs, U: STaskRs> TMeasure<T, U> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTMeasure<'_, T, U> {
		LockGuardForTMeasure {
			c_task: self.c_task,
			c_taskmig: self.c_taskmig,
		}
	}
}
