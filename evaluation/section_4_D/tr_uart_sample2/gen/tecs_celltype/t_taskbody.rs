use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_dataqueue_rs::*};

pub struct TTaskbody<T, U>
where
	T: SXUartMeasure + 'static,
	U: SDataqueueRs + 'static,
{
	c_x_uart: &'static T,
	c_dataqueue: &'static U,
}

pub struct ETaskbodyForTTaskbody {
	pub cell: &'static TTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs>,
}

pub struct LockGuardForTTaskbody<'a, T, U>
where
	T: SXUartMeasure + 'static,
	U: SDataqueueRs + 'static,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_TASKBODY: TTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs> = TTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY,
};

impl<T: SXUartMeasure, U: SDataqueueRs> TTaskbody<T, U> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskbody<'_, T, U> {
		LockGuardForTTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
		}
	}
}
