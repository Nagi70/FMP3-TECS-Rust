use crate::{s_task_rs::*, t_task_rs::*, s_semaphore_rs::*, t_semaphore_rs::*};

pub struct TMeasure<'a, T, U, V>where
	T: STaskRs,
	U: STaskRs,
	V: SSemaphoreRs,
{
	c_task: &'a T,
	c_taskmig: &'a U,
	c_semaphore: &'a V,
}

pub struct ETaskbodyForTMeasure<'a>{
	pub cell: &'a TMeasure<'a, ETaskForTTaskRs<'a>, ETaskForTTaskRs<'a>, ESemaphoreForTSemaphoreRs<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_MEASURE: TMeasure<ETaskForTTaskRs, ETaskForTTaskRs, ESemaphoreForTSemaphoreRs> = TMeasure {
	c_task: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2,
	c_taskmig: &ETASKFORRPROCESSORALLMIG_TASKMIG,
	c_semaphore: &ESEMAPHOREFORRPROCESSORALLMIG_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_MEASURE: ETaskbodyForTMeasure = ETaskbodyForTMeasure {
	cell: &RPROCESSOR1SYMMETRIC_MEASURE,
};

impl<T: STaskRs, U: STaskRs, V: SSemaphoreRs> TMeasure<'_, T, U, V> {
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static U, &'static V) {
		(
			self.c_task,
			self.c_taskmig,
			self.c_semaphore
		)
	}
}
