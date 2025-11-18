use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_led::*, si_dataqueue_rs::*, s_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_mio_led::*, t_dataqueue_rs::*};

pub struct TXUartTaskbody<T, U, V, W>
where
	T: SXUartMeasure + 'static,
	U: SLed + 'static,
	V: SiDataqueueRs + 'static,
	W: SDataqueueRs + 'static,
{
	c_x_uart: &'static T,
	c_led: &'static U,
	c_dataqueue: &'static V,
	c_dataqueue_led: &'static W,
}

pub struct ETaskbodyForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody<EXUartForTXUart, ELedForTMioLed, EiDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs>,
}

pub struct EXUartMainForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody<EXUartForTXUart, ELedForTMioLed, EiDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs>,
}

pub struct LockGuardForTXUartTaskbody<'a, T, U, V, W>
where
	T: SXUartMeasure + 'static,
	U: SLed + 'static,
	V: SiDataqueueRs + 'static,
	W: SDataqueueRs + 'static,
{
	pub c_x_uart: &'a T,
	pub c_led: &'a U,
	pub c_dataqueue: &'a V,
	pub c_dataqueue_led: &'a W,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXUartTaskbody<EXUartForTXUart, ELedForTMioLed, EiDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs> = TXUartTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_led: &ELEDFORRPROCESSOR1SYMMETRIC_LED,
	c_dataqueue: &EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	c_dataqueue_led: &EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: ETaskbodyForTXUartTaskbody = ETaskbodyForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

#[unsafe(link_section = ".rodata")]
pub static EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: EXUartMainForTXUartTaskbody = EXUartMainForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

impl<T: SXUartMeasure, U: SLed, V: SiDataqueueRs, W: SDataqueueRs> TXUartTaskbody<T, U, V, W> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUartTaskbody<'_, T, U, V, W> {
		LockGuardForTXUartTaskbody {
			c_x_uart: self.c_x_uart,
			c_led: self.c_led,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
		}
	}
}
