use itron::abi::*;
use crate::kernel_cfg::*;

use crate::{si_handler_body::*, t_xuart::*};

pub struct TIsrRs<'a, T>where
	T: SiHandlerBody,
{
	pub ci_isr_body: &'a T,
	id: ID,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTISR: TIsrRs<EiHandlerBodyForTXuart> = TIsrRs {
	ci_isr_body: &EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART,
	id: ISRID_PRC2,
};

