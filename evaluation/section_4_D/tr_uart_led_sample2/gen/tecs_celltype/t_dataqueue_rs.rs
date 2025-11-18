use itron::dataqueue::DataqueueRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
pub struct TDataqueueRs{
	dataqueue_ref: DataqueueRef<'static>,
}

pub struct EDataqueueForTDataqueueRs {
	pub cell: &'static TDataqueueRs,
}

pub struct EiDataqueueForTDataqueueRs {
	pub cell: &'static TDataqueueRs,
}

pub struct LockGuardForTDataqueueRs<'a>{
	pub dataqueue_ref: &'a DataqueueRef<'static>,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_DATAQUEUE: TDataqueueRs = TDataqueueRs {
	dataqueue_ref: unsafe{DataqueueRef::from_raw_nonnull(NonZeroI32::new(DTQID_UART).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EDataqueueForTDataqueueRs = EDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
pub static EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EiDataqueueForTDataqueueRs = EiDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_DATAQUEUELED: TDataqueueRs = TDataqueueRs {
	dataqueue_ref: unsafe{DataqueueRef::from_raw_nonnull(NonZeroI32::new(DTQID_LED).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED: EDataqueueForTDataqueueRs = EDataqueueForTDataqueueRs {
	cell: &RPROCESSOR2SYMMETRIC_DATAQUEUELED,
};

#[unsafe(link_section = ".rodata")]
pub static EIDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED: EiDataqueueForTDataqueueRs = EiDataqueueForTDataqueueRs {
	cell: &RPROCESSOR2SYMMETRIC_DATAQUEUELED,
};

impl TDataqueueRs {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTDataqueueRs<'_> {
		LockGuardForTDataqueueRs {
			dataqueue_ref: &self.dataqueue_ref,
		}
	}
}
