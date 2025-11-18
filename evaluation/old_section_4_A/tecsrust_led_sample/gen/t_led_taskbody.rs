use crate::{s_led::*, t_mio_led::*};

pub struct TLedTaskbody<'a, T>
where
	T: SLed,
{
	c_led: &'a T,
}

pub struct ETaskbodyForTLedTaskbody<'a>{
	pub cell: &'a TLedTaskbody<'a, ELedForTMioLed<'a>>,
}

pub struct LockGuardForTLedTaskbody<'a, T>
where
	T: SLed,
{
	pub c_led: &'a T,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_LEDTASKBODY: TLedTaskbody<ELedForTMioLed> = TLedTaskbody {
	c_led: &ELEDFORRPROCESSOR1SYMMETRIC_MIOLED,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_LEDTASKBODY: ETaskbodyForTLedTaskbody = ETaskbodyForTLedTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_LEDTASKBODY,
};

impl<T: SLed> TLedTaskbody<'_, T> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTLedTaskbody<'_, T> {
		LockGuardForTLedTaskbody {
			c_led: self.c_led,
		}
	}
}
