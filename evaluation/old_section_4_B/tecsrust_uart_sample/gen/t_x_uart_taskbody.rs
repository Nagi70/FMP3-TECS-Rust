use crate::{s_x_uart_measure::*, t_x_uart::*, si_dataqueue_rs::*, t_dataqueue_rs::*};

pub struct TXUartTaskbody<'a, T, U>
where
	T: SXUartMeasure,
	U: SiDataqueueRs,
{
	c_x_uart: &'a T,
	c_dataqueue: &'a U,
}

pub struct ETaskbodyForTXUartTaskbody<'a>{
	pub cell: &'a TXUartTaskbody<'a, EXUartForTXUart<'a>, EiDataqueueForTDataqueueRs<'a>>,
}

pub struct EXUartMainForTXUartTaskbody<'a>{
	pub cell: &'a TXUartTaskbody<'a, EXUartForTXUart<'a>, EiDataqueueForTDataqueueRs<'a>>,
}

pub struct LockGuardForTXUartTaskbody<'a, T, U>
where
	T: SXUartMeasure,
	U: SiDataqueueRs,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXUartTaskbody<EXUartForTXUart, EiDataqueueForTDataqueueRs> = TXUartTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: ETaskbodyForTXUartTaskbody = ETaskbodyForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

#[link_section = ".rodata"]
pub static EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: EXUartMainForTXUartTaskbody = EXUartMainForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

impl<T: SXUartMeasure, U: SiDataqueueRs> TXUartTaskbody<'_, T, U> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUartTaskbody<'_, T, U> {
		LockGuardForTXUartTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
		}
	}
}
