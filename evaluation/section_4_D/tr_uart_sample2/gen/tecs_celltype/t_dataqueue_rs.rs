use itron::dataqueue::DataqueueRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
pub struct TDataqueueRs{
	dataqueue_ref: itron::dataqueue::DataqueueRef<'static>,
}

pub struct EDataqueueForTDataqueueRs {
	pub cell: &'static TDataqueueRs,
}

pub struct EiDataqueueForTDataqueueRs {
	pub cell: &'static TDataqueueRs,
}

pub struct LockGuardForTDataqueueRs<'a>{
	pub dataqueue_ref: &'a itron::dataqueue::DataqueueRef<'static>,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_DATAQUEUE: TDataqueueRs = TDataqueueRs {
	dataqueue_ref: unsafe{itron::dataqueue::DataqueueRef::from_raw_nonnull(NonZeroI32::new(DTQID_UART).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EDataqueueForTDataqueueRs = EDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
pub static EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EiDataqueueForTDataqueueRs = EiDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

impl TDataqueueRs {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTDataqueueRs<'_> {
		LockGuardForTDataqueueRs {
			dataqueue_ref: &self.dataqueue_ref,
		}
	}
}
