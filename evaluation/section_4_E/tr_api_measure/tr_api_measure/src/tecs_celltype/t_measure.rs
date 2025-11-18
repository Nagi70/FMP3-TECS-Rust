use crate::tecs_global::*;
use crate::tecs_signature::s_task_rs::*;
use crate::tecs_celltype::t_task_rs::*;
pub struct TMeasure<T>
where
	T: STaskRs + 'static,
{
	c_task: &'static T,
}

pub struct ETaskbodyForTMeasure {
	pub cell: &'static TMeasure<ETaskForTTaskRs>,
}

pub struct LockGuardForTMeasure<'a, T>
where
	T: STaskRs + 'static,
{
	pub c_task: &'a T,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_MEASURE: TMeasure<ETaskForTTaskRs> = TMeasure {
	c_task: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_1,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_MEASURE: ETaskbodyForTMeasure = ETaskbodyForTMeasure {
	cell: &RPROCESSOR1SYMMETRIC_MEASURE,
};

impl<T: STaskRs> TMeasure<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTMeasure<'_, T> {
		LockGuardForTMeasure {
			c_task: self.c_task,
		}
	}
}
