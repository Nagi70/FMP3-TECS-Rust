use crate::{s_task_rs::*, t_task_rs::*, s_semaphore_rs::*, t_semaphore_rs::*};

pub struct TReset<'a, T, U, V, W>
where
	T: STaskRs,
	U: STaskRs,
	V: STaskRs,
	W: SSemaphoreRs,
{
	c_task: &'a T,
	c_taskself: &'a U,
	c_taskmig: &'a V,
	c_semaphore: &'a W,
}

pub struct EResetForTReset<'a>{
	pub cell: &'a TReset<'a, ETaskForTTaskRs<'a>, ETaskForTTaskRs<'a>, ETaskForTTaskRs<'a>, ESemaphoreForTSemaphoreRs<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_RESET: TReset<ETaskForTTaskRs, ETaskForTTaskRs, ETaskForTTaskRs, ESemaphoreForTSemaphoreRs> = TReset {
	c_task: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2,
	c_taskself: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_1,
	c_taskmig: &ETASKFORRPROCESSORALLMIG_TASKMIG,
	c_semaphore: &ESEMAPHOREFORRPROCESSORALLMIG_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static ERESETFORRPROCESSOR2SYMMETRIC_RESET: EResetForTReset = EResetForTReset {
	cell: &RPROCESSOR2SYMMETRIC_RESET,
};

impl<T: STaskRs, U: STaskRs, V: STaskRs, W: SSemaphoreRs> TReset<'_, T, U, V, W> {
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static U, &'static V, &'static W) {
		(
			self.c_task,
			self.c_taskself,
			self.c_taskmig,
			self.c_semaphore
		)
	}
}
