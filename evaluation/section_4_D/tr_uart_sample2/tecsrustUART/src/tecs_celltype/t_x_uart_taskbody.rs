use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart_measure::*, si_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_dataqueue_rs::*};

pub struct TXUartTaskbody<T, U>
where
	T: SXUartMeasure + 'static,
	U: SiDataqueueRs + 'static,
{
	c_x_uart: &'static T,
	c_dataqueue: &'static U,
}

pub struct ETaskbodyForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody<EXUartForTXUart, EiDataqueueForTDataqueueRs>,
}

pub struct EXUartMainForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody<EXUartForTXUart, EiDataqueueForTDataqueueRs>,
}

pub struct LockGuardForTXUartTaskbody<'a, T, U>
where
	T: SXUartMeasure + 'static,
	U: SiDataqueueRs + 'static,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXUartTaskbody<EXUartForTXUart, EiDataqueueForTDataqueueRs> = TXUartTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: ETaskbodyForTXUartTaskbody = ETaskbodyForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

#[unsafe(link_section = ".rodata")]
pub static EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: EXUartMainForTXUartTaskbody = EXUartMainForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

impl<T: SXUartMeasure, U: SiDataqueueRs> TXUartTaskbody<T, U> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUartTaskbody<'_, T, U> {
		LockGuardForTXUartTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
		}
	}
}
