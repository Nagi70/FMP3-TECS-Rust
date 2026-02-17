use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_led::*, si_dataqueue_rs::*, s_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_mio_led::*, t_dataqueue_rs::*};

pub struct TXUartTaskbody{
	c_x_uart: &'static EXUartForTXUart<ConfigUart>,
	c_led: &'static ELedForTMioLed<ConfigLed>,
	c_dataqueue: &'static EiDataqueueForTDataqueueRs<ConfigDataqueue>,
	c_dataqueue_led: &'static EDataqueueForTDataqueueRs<ConfigDataqueueled>,
}

pub struct ETaskbodyForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody,
}

pub struct EXUartMainForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody,
}

pub struct LockGuardForTXUartTaskbody<'a>{
	pub c_x_uart: &'a EXUartForTXUart<ConfigUart>,
	pub c_led: &'a ELedForTMioLed<ConfigLed>,
	pub c_dataqueue: &'a EiDataqueueForTDataqueueRs<ConfigDataqueue>,
	pub c_dataqueue_led: &'a EDataqueueForTDataqueueRs<ConfigDataqueueled>,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_UARTTASKBODY: TXUartTaskbody = TXUartTaskbody {
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

impl TXUartTaskbody {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUartTaskbody {
		LockGuardForTXUartTaskbody {
			c_x_uart: self.c_x_uart,
			c_led: self.c_led,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
		}
	}
}
