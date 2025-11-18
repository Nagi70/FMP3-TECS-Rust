use crate::{s_xuart_measure::*, t_xuart::*};

pub struct TXuartTaskbody<'a, T>
where
	T: SXuartMeasure,
{
	c_xuart: &'a T,
}

pub struct ETaskbodyForTXuartTaskbody<'a>{
	pub cell: &'a TXuartTaskbody<'a, EXuartForTXuart<'a>>,
}

pub struct LockGuardForTXuartTaskbody<'a, T>
where
	T: SXuartMeasure,
{
	pub c_xuart: &'a T,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXuartTaskbody<EXuartForTXuart> = TXuartTaskbody {
	c_xuart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: ETaskbodyForTXuartTaskbody = ETaskbodyForTXuartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

impl<T: SXuartMeasure> TXuartTaskbody<'_, T> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXuartTaskbody<'_, T> {
		LockGuardForTXuartTaskbody {
			c_xuart: self.c_xuart,
		}
	}
}
