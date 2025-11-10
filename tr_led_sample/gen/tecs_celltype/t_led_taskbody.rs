use crate::tecs_global::*;
use crate::tecs_signature::s_led::*;
use crate::tecs_celltype::t_mio_led::*;
pub struct TLedTaskbody<T>
where
	T: SLed + 'static,
{
	c_led: &'static T,
}

pub struct ETaskbodyForTLedTaskbody {
	pub cell: &'static TLedTaskbody<ELedForTMioLed>,
}

pub struct LockGuardForTLedTaskbody<'a, T>
where
	T: SLed + 'static,
{
	pub c_led: &'a T,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_LEDTASKBODY: TLedTaskbody<ELedForTMioLed> = TLedTaskbody {
	c_led: &ELEDFORRPROCESSOR1SYMMETRIC_MIOLED,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_LEDTASKBODY: ETaskbodyForTLedTaskbody = ETaskbodyForTLedTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_LEDTASKBODY,
};

impl<T: SLed> TLedTaskbody<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTLedTaskbody<'_, T> {
		LockGuardForTLedTaskbody {
			c_led: self.c_led,
		}
	}
}
