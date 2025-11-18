use crate::{s_xuart_measure::*, t_xuart::*};

use crate::{s_xuart_measure::*, t_xuart::*, si_dataqueue_rs::*, t_dataqueue_rs::*};

pub struct TXuartTaskbody<'a, T, U>
where
	T: SXuartMeasure,
	U: SiDataqueueRs,
{
	c_xuart: &'a T,
	c_dataqueue: &'a U,
}

pub struct ETaskbodyForTXuartTaskbody<'a>{
	pub cell: &'a TXuartTaskbody<'a, EXuartForTXuart<'a>, EiDataqueueForTDataqueueRs<'a>>,
}

pub struct EXuartMainForTXuartTaskbody<'a>{
	pub cell: &'a TXuartTaskbody<'a, EXuartForTXuart<'a>, EiDataqueueForTDataqueueRs<'a>>,
}

pub struct LockGuardForTXuartTaskbody<'a, T, U>
where
	T: SXuartMeasure,
	U: SiDataqueueRs,
{
	pub c_xuart: &'a T,
	pub c_dataqueue: &'a U,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXuartTaskbody<EXuartForTXuart, EiDataqueueForTDataqueueRs> = TXuartTaskbody {
	c_xuart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: ETaskbodyForTXuartTaskbody = ETaskbodyForTXuartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

#[link_section = ".rodata"]
pub static EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: EXuartMainForTXuartTaskbody = EXuartMainForTXuartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

impl<T: SXuartMeasure, U: SiDataqueueRs> TXuartTaskbody<'_, T, U> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXuartTaskbody<'_, T, U> {
		LockGuardForTXuartTaskbody {
			c_xuart: self.c_xuart,
			c_dataqueue: self.c_dataqueue,
		}
	}
}
