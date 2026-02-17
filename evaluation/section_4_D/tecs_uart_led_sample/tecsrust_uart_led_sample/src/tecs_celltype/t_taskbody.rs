use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_dataqueue_rs::*};

pub struct TTaskbody{
	c_x_uart: &'static EXUartForTXUart<ConfigUart>,
	c_dataqueue: &'static EDataqueueForTDataqueueRs<ConfigDataqueue>,
	c_dataqueue_led: &'static EDataqueueForTDataqueueRs<ConfigDataqueueled>,
	variable: &'static SyncTTaskbodyVar,
}

pub struct TTaskbodyVar {
	pub buffer: &'static mut [u32],
	pub buffer_count: u32,
}

pub struct SyncTTaskbodyVar {
	unsafe_var: UnsafeCell<TTaskbodyVar>,
}

unsafe impl Sync for SyncTTaskbodyVar {}

pub struct ETaskbodyForTTaskbody {
	pub cell: &'static TTaskbody,
}

pub struct LockGuardForTTaskbody<'a>{
	pub c_x_uart: &'a EXUartForTXUart<ConfigUart>,
	pub c_dataqueue: &'a EDataqueueForTDataqueueRs<ConfigDataqueue>,
	pub c_dataqueue_led: &'a EDataqueueForTDataqueueRs<ConfigDataqueueled>,
	pub var: &'a mut TTaskbodyVar,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_TASKBODY: TTaskbody = TTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	c_dataqueue_led: &EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED,
	variable: &RPROCESSOR2SYMMETRIC_TASKBODYVAR,
};

static RPROCESSOR2SYMMETRIC_TASKBODYVAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
		buffer: unsafe{ &mut *core::ptr::addr_of_mut!(RPROCESSOR2SYMMETRIC_TASKBODYVARARRAY1) },
	buffer_count: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY,
};

static mut RPROCESSOR2SYMMETRIC_TASKBODYVARARRAY1: [u32; 8] = [0; 8];

impl TTaskbody {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskbody<'_> {
		LockGuardForTTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
		}
	}
}
