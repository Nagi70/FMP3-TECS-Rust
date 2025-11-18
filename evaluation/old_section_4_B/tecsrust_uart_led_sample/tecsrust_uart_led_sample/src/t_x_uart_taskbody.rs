use crate::{s_x_uart_measure::*, t_x_uart::*, s_led::*, t_mio_led::*, si_dataqueue_rs::*, t_dataqueue_rs::*, s_dataqueue_rs::*};

pub struct TXUartTaskbody<'a, T, U, V, W>
where
	T: SXUartMeasure,
	U: SLed,
	V: SiDataqueueRs,
	W: SDataqueueRs,
{
	c_x_uart: &'a T,
	c_led: &'a U,
	c_dataqueue: &'a V,
	c_dataqueue_led: &'a W,
}

pub struct ETaskbodyForTXUartTaskbody<'a>{
	pub cell: &'a TXUartTaskbody<'a, EXUartForTXUart<'a>, ELedForTMioLed<'a>, EiDataqueueForTDataqueueRs<'a>, EDataqueueForTDataqueueRs<'a>>,
}

pub struct EXUartMainForTXUartTaskbody<'a>{
	pub cell: &'a TXUartTaskbody<'a, EXUartForTXUart<'a>, ELedForTMioLed<'a>, EiDataqueueForTDataqueueRs<'a>, EDataqueueForTDataqueueRs<'a>>,
}

pub struct LockGuardForTXUartTaskbody<'a, T, U, V, W>
where
	T: SXUartMeasure,
	U: SLed,
	V: SiDataqueueRs,
	W: SDataqueueRs,
{
	pub c_x_uart: &'a T,
	pub c_led: &'a U,
	pub c_dataqueue: &'a V,
	pub c_dataqueue_led: &'a W,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXUartTaskbody<EXUartForTXUart, ELedForTMioLed, EiDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs> = TXUartTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_led: &ELEDFORRPROCESSOR1SYMMETRIC_LED,
	c_dataqueue: &EIDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	c_dataqueue_led: &EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: ETaskbodyForTXUartTaskbody = ETaskbodyForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

#[link_section = ".rodata"]
pub static EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY: EXUartMainForTXUartTaskbody = EXUartMainForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_UARTTASKBODY,
};

impl<T: SXUartMeasure, U: SLed, V: SiDataqueueRs, W: SDataqueueRs> TXUartTaskbody<'_, T, U, V, W> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUartTaskbody<'_, T, U, V, W> {
		LockGuardForTXUartTaskbody {
			c_x_uart: self.c_x_uart,
			c_led: self.c_led,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
		}
	}
}
