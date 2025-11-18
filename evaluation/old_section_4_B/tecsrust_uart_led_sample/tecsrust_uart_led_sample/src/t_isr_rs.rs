use itron::abi::ID;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::{si_handler_body::*, t_x_uart::*};

pub struct TIsrRs<'a, T>
where
	T: SiHandlerBody,
{
	pub ci_isr_body: &'a T,
	id: ID,
}

pub struct LockGuardForTIsrRs<'a, T>
where
	T: SiHandlerBody,
{
	pub ci_isr_body: &'a T,
	pub id: &'a ID,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTISR: TIsrRs<EiHandlerBodyForTXUart> = TIsrRs {
	ci_isr_body: &EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART,
	id: ISRID_PRC2,
};

