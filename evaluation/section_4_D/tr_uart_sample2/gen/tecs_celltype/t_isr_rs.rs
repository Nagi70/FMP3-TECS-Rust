use itron::abi::ID;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::si_handler_body::*;
use crate::tecs_celltype::t_x_uart::*;
pub struct TIsrRs<T>
where
	T: SiHandlerBody + 'static,
{
	pub ci_isr_body: &'static T,
	id: ID,
}

pub struct LockGuardForTIsrRs<'a, T>
where
	T: SiHandlerBody + 'static,
{
	pub ci_isr_body: &'a T,
	pub id: &'a ID,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_UARTISR: TIsrRs<EiHandlerBodyForTXUart> = TIsrRs {
	ci_isr_body: &EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART,
	id: ISRID_PRC2,
};

