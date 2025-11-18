use itron::semaphore::SemaphoreRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
pub struct TSemaphoreRs<'a>{
	semaphore_ref: SemaphoreRef<'a>,
}

pub struct ESemaphoreForTSemaphoreRs<'a>{
	pub cell: &'a TSemaphoreRs<'a>,
}

pub struct EiSemaphoreForTSemaphoreRs<'a>{
	pub cell: &'a TSemaphoreRs<'a>,
}

pub struct EiNotificationHandlerForTSemaphoreRs<'a>{
	pub cell: &'a TSemaphoreRs<'a>,
}

#[link_section = ".rodata"]
pub static RPROCESSORALLMIG_SEMAPHORE: TSemaphoreRs = TSemaphoreRs {
	semaphore_ref: unsafe{SemaphoreRef::from_raw_nonnull(NonZeroI32::new(SEMID_1).unwrap())},
};

#[link_section = ".rodata"]
pub static ESEMAPHOREFORRPROCESSORALLMIG_SEMAPHORE: ESemaphoreForTSemaphoreRs = ESemaphoreForTSemaphoreRs {
	cell: &RPROCESSORALLMIG_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static EISEMAPHOREFORRPROCESSORALLMIG_SEMAPHORE: EiSemaphoreForTSemaphoreRs = EiSemaphoreForTSemaphoreRs {
	cell: &RPROCESSORALLMIG_SEMAPHORE,
};

#[link_section = ".rodata"]
pub static EINOTIFICATIONHANDLERFORRPROCESSORALLMIG_SEMAPHORE: EiNotificationHandlerForTSemaphoreRs = EiNotificationHandlerForTSemaphoreRs {
	cell: &RPROCESSORALLMIG_SEMAPHORE,
};

impl<> TSemaphoreRs<'_> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> &'static SemaphoreRef {
		&self.semaphore_ref
	}
}
