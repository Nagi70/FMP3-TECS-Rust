use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::{s_x_uart_measure::*, t_x_uart::*, s_dataqueue_rs::*, t_dataqueue_rs::*};

pub struct TTaskbody<'a, T, U, V>
where
	T: SXUartMeasure,
	U: SDataqueueRs,
	V: SDataqueueRs,
{
	c_x_uart: &'a T,
	c_dataqueue: &'a U,
	c_dataqueue_led: &'a V,
	variable: &'a SyncTTaskbodyVar,
}

pub struct TTaskbodyVar{
	pub buffer: [u32; 8],
	pub buffer_count: u32,
}

pub struct SyncTTaskbodyVar{
	unsafe_var: UnsafeCell<TTaskbodyVar>,
}

unsafe impl Sync for SyncTTaskbodyVar {}

pub struct ETaskbodyForTTaskbody<'a>{
	pub cell: &'a TTaskbody<'a, EXUartForTXUart<'a>, EDataqueueForTDataqueueRs<'a>, EDataqueueForTDataqueueRs<'a>>,
}

pub struct LockGuardForTTaskbody<'a, T, U, V>
where
	T: SXUartMeasure,
	U: SDataqueueRs,
	V: SDataqueueRs,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
	pub c_dataqueue_led: &'a V,
	pub var: &'a mut TTaskbodyVar,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_TASKBODY: TTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs> = TTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	c_dataqueue_led: &EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED,
	variable: &RPROCESSOR2SYMMETRIC_TASKBODYVAR,
};

pub static RPROCESSOR2SYMMETRIC_TASKBODYVAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
		buffer: [0, 0, 0, 0, 0, 0, 0, 0],
		buffer_count: 0,
	}),
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY,
};

impl<T: SXUartMeasure, U: SDataqueueRs, V: SDataqueueRs> TTaskbody<'_, T, U, V> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskbody<'_, T, U, V> {
		LockGuardForTTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
		}
	}
}
