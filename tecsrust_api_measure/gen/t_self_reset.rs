use crate::{s_task_rs::*, t_task_rs::*, s_semaphore_rs::*, t_semaphore_rs::*};

pub struct TSelfReset<'a, T, U>
where
	T: STaskRs,
	U: SSemaphoreRs,
{
	c_taskself: &'a T,
	c_semaphore: &'a U,
}

pub struct ESelfResetForTSelfReset<'a>{
	pub cell: &'a TSelfReset<'a, ETaskForTTaskRs<'a>, ESemaphoreForTSemaphoreRs<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_SELFRESET: TSelfReset<ETaskForTTaskRs, ESemaphoreForTSemaphoreRs> = TSelfReset {
	c_taskself: &ETASKFORRPROCESSOR2SYMMETRIC_TASK2_2,
	c_semaphore: &ESEMAPHOREFORRPROCESSORALLMIG_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static ESELFRESETFORRPROCESSOR2SYMMETRIC_SELFRESET: ESelfResetForTSelfReset = ESelfResetForTSelfReset {
	cell: &RPROCESSOR2SYMMETRIC_SELFRESET,
};

impl<T: STaskRs, U: SSemaphoreRs> TSelfReset<'_, T, U> {
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static U) {
		(
			self.c_taskself,
			self.c_semaphore
		)
	}
}
