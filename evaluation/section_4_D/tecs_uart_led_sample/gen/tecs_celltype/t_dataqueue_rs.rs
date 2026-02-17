use itron::dataqueue::DataqueueRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
pub trait TDataqueueRsConfig: 'static {
    const DATAQUEUE_REF: itron::dataqueue::DataqueueRef<'static>;
}

pub struct TDataqueueRsDataqueueRef<CONFIG: TDataqueueRsConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TDataqueueRsConfig> core::ops::Deref for TDataqueueRsDataqueueRef<CONFIG> {
    type Target = itron::dataqueue::DataqueueRef<'static>;
    #[inline(always)]
    fn deref(&self) -> &itron::dataqueue::DataqueueRef<'static> {
        &CONFIG::DATAQUEUE_REF
    }
}

// Instance Configurations
pub struct ConfigDataqueue;
impl TDataqueueRsConfig for ConfigDataqueue {
    const DATAQUEUE_REF: itron::dataqueue::DataqueueRef<'static> = unsafe{DataqueueRef::from_raw_nonnull(NonZeroI32::new(DTQID_UART).unwrap())};
}

pub struct ConfigDataqueueled;
impl TDataqueueRsConfig for ConfigDataqueueled {
    const DATAQUEUE_REF: itron::dataqueue::DataqueueRef<'static> = unsafe{DataqueueRef::from_raw_nonnull(NonZeroI32::new(DTQID_LED).unwrap())};
}

pub struct TDataqueueRs<CONFIG>{
	_phantom: core::marker::PhantomData<CONFIG>,
}

pub struct EDataqueueForTDataqueueRs<CONFIG: TDataqueueRsConfig> {
	pub cell: &'static TDataqueueRs<CONFIG>,
}

pub struct EiDataqueueForTDataqueueRs<CONFIG: TDataqueueRsConfig> {
	pub cell: &'static TDataqueueRs<CONFIG>,
}

pub struct LockGuardForTDataqueueRs<CONFIG: TDataqueueRsConfig>{
	pub dataqueue_ref: TDataqueueRsDataqueueRef<CONFIG>,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_DATAQUEUE: TDataqueueRs<ConfigDataqueue> = TDataqueueRs {
	_phantom: core::marker::PhantomData,
};

#[unsafe(link_section = ".rodata")]
pub static EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EDataqueueForTDataqueueRs<ConfigDataqueue> = EDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
pub static EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE: EiDataqueueForTDataqueueRs<ConfigDataqueue> = EiDataqueueForTDataqueueRs {
	cell: &RPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_DATAQUEUELED: TDataqueueRs<ConfigDataqueueled> = TDataqueueRs {
	_phantom: core::marker::PhantomData,
};

#[unsafe(link_section = ".rodata")]
pub static EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED: EDataqueueForTDataqueueRs<ConfigDataqueueled> = EDataqueueForTDataqueueRs {
	cell: &RPROCESSOR2SYMMETRIC_DATAQUEUELED,
};

#[unsafe(link_section = ".rodata")]
pub static EIDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED: EiDataqueueForTDataqueueRs<ConfigDataqueueled> = EiDataqueueForTDataqueueRs {
	cell: &RPROCESSOR2SYMMETRIC_DATAQUEUELED,
};

impl<CONFIG: TDataqueueRsConfig> TDataqueueRs<CONFIG> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTDataqueueRs<CONFIG> {
		LockGuardForTDataqueueRs {
			dataqueue_ref: TDataqueueRsDataqueueRef(core::marker::PhantomData),
		}
	}
}
