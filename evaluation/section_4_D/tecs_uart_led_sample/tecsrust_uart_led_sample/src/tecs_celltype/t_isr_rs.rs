use itron::abi::ID;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::si_handler_body::*;
use crate::tecs_celltype::t_x_uart::*;
pub trait TIsrRsConfig: 'static {
    const ID: ID;
}

pub struct TIsrRsId<CONFIG: TIsrRsConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TIsrRsConfig> core::ops::Deref for TIsrRsId<CONFIG> {
    type Target = ID;
    #[inline(always)]
    fn deref(&self) -> &ID {
        &CONFIG::ID
    }
}

// Instance Configurations
pub struct ConfigUartisr;
impl TIsrRsConfig for ConfigUartisr {
    const ID: ID = ISRID_PRC2;
}

pub struct TIsrRs<CONFIG>
where
	CONFIG: TIsrRsConfig,
{
	pub ci_isr_body: &'static EiHandlerBodyForTXUart<ConfigUart>,
	_phantom: core::marker::PhantomData<CONFIG>,
}

pub struct LockGuardForTIsrRs<'a, CONFIG: TIsrRsConfig>
where
	CONFIG: TIsrRsConfig,
{
	pub ci_isr_body: &'a EiHandlerBodyForTXUart<ConfigUart>,
	pub id: TIsrRsId<CONFIG>,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_UARTISR: TIsrRs<ConfigUartisr> = TIsrRs {
	ci_isr_body: &EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART,
	_phantom: core::marker::PhantomData,
};

