use crate::{s_xuart_measure::*, t_xuart::*, s_dataqueue_rs::*, t_dataqueue_rs::*};

use crate::{s_x_uart_measure::*, t_x_uart::*, s_dataqueue_rs::*, t_dataqueue_rs::*};

pub struct TTaskbody<'a, T, U>
where
	T: SXUartMeasure,
	U: SDataqueueRs,
{
	c_x_uart: &'a T,
	c_dataqueue: &'a U,
}

pub struct ETaskbodyForTTaskbody<'a>{
	pub cell: &'a TTaskbody<'a, EXUartForTXUart<'a>, EDataqueueForTDataqueueRs<'a>>,
}

pub struct LockGuardForTTaskbody<'a, T, U>
where
	T: SXUartMeasure,
	U: SDataqueueRs,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASKBODY: TTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs> = TTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY,
};

impl<T: SXUartMeasure, U: SDataqueueRs> TTaskbody<'_, T, U> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskbody<'_, T, U> {
		LockGuardForTTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
		}
	}
}
