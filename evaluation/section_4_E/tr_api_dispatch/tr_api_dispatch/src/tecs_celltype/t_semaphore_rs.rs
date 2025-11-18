use itron::semaphore::SemaphoreRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
pub struct TSemaphoreRs{
	semaphore_ref: itron::semaphore::SemaphoreRef<'static>,
}

pub struct ESemaphoreForTSemaphoreRs {
	pub cell: &'static TSemaphoreRs,
}

pub struct EiSemaphoreForTSemaphoreRs {
	pub cell: &'static TSemaphoreRs,
}

pub struct LockGuardForTSemaphoreRs<'a>{
	pub semaphore_ref: &'a itron::semaphore::SemaphoreRef<'static>,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSORALLMIG_SEMAPHORE: TSemaphoreRs = TSemaphoreRs {
	semaphore_ref: unsafe{SemaphoreRef::from_raw_nonnull(NonZeroI32::new(SEMID_1).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ESEMAPHOREFORRPROCESSORALLMIG_SEMAPHORE: ESemaphoreForTSemaphoreRs = ESemaphoreForTSemaphoreRs {
	cell: &RPROCESSORALLMIG_SEMAPHORE,
};

#[unsafe(link_section = ".rodata")]
pub static EISEMAPHOREFORRPROCESSORALLMIG_SEMAPHORE: EiSemaphoreForTSemaphoreRs = EiSemaphoreForTSemaphoreRs {
	cell: &RPROCESSORALLMIG_SEMAPHORE,
};

impl TSemaphoreRs {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTSemaphoreRs<'_> {
		LockGuardForTSemaphoreRs {
			semaphore_ref: &self.semaphore_ref,
		}
	}
}
