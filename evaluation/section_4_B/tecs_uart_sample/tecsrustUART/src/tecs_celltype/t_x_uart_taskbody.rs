use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart::*, s_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_dataqueue_rs::*};

pub struct TXUartTaskbody<T, U>
where
	T: SXUart + 'static,
	U: SDataqueueRs + 'static,
{
	c_x_uart: &'static T,
	c_dataqueue: &'static U,
	buffer_size: u8,
	variable: &'static SyncTXUartTaskbodyVar,
}

pub struct TXUartTaskbodyVar {
	pub buffer: &'static mut [u8],
	pub buffer_count: u8,
}

pub struct SyncTXUartTaskbodyVar {
	unsafe_var: UnsafeCell<TXUartTaskbodyVar>,
}

unsafe impl Sync for SyncTXUartTaskbodyVar {}

pub struct ETaskbodyForTXUartTaskbody {
	pub cell: &'static TXUartTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs>,
}

pub struct LockGuardForTXUartTaskbody<'a, T, U>
where
	T: SXUart + 'static,
	U: SDataqueueRs + 'static,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
	pub buffer_size: &'a u8,
	pub var: &'a mut TXUartTaskbodyVar,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_TASKBODY: TXUartTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs> = TXUartTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	buffer_size: 128,
	variable: &RPROCESSOR1SYMMETRIC_TASKBODYVAR,
};

static RPROCESSOR1SYMMETRIC_TASKBODYVAR: SyncTXUartTaskbodyVar = SyncTXUartTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TXUartTaskbodyVar {
		buffer: unsafe{ &mut *core::ptr::addr_of_mut!(RPROCESSOR1SYMMETRIC_TASKBODYVARARRAY1) },
	buffer_count: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY: ETaskbodyForTXUartTaskbody = ETaskbodyForTXUartTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_TASKBODY,
};

static mut RPROCESSOR1SYMMETRIC_TASKBODYVARARRAY1: [u8; 128] = [0; 128];

impl<T: SXUart, U: SDataqueueRs> TXUartTaskbody<T, U> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUartTaskbody<'_, T, U> {
		LockGuardForTXUartTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
			buffer_size: &self.buffer_size,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
		}
	}
}
