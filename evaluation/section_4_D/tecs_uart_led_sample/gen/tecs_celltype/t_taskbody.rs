use itron::mutex::MutexRef;
use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_dataqueue_rs::*};

use crate::tecs_celltype::{t_x_uart::*, t_dataqueue_rs::*};

pub struct TTaskbody<T, U, V>
where
	T: SXUartMeasure + 'static,
	U: SDataqueueRs + 'static,
	V: SDataqueueRs + 'static,
{
	c_x_uart: &'static T,
	c_dataqueue: &'static U,
	c_dataqueue_led: &'static V,
	variable: &'static SyncTTaskbodyVar,
	ex_ctrl_ref: &'static TECSMutexRef,
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
	pub cell: &'static TTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs>,
}

pub struct LockGuardForTTaskbody<'a, T, U, V>
where
	T: SXUartMeasure + 'static,
	U: SDataqueueRs + 'static,
	V: SDataqueueRs + 'static,
{
	pub c_x_uart: &'a T,
	pub c_dataqueue: &'a U,
	pub c_dataqueue_led: &'a V,
	pub var: &'a mut TTaskbodyVar,
	ex_ctrl_ref: &'static TECSMutexRef,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_TASKBODY: TTaskbody<EXUartForTXUart, EDataqueueForTDataqueueRs, EDataqueueForTDataqueueRs> = TTaskbody {
	c_x_uart: &EXUARTFORRPROCESSOR1SYMMETRIC_UART,
	c_dataqueue: &EDATAQUEUEFORRPROCESSOR1SYMMETRIC_DATAQUEUE,
	c_dataqueue_led: &EDATAQUEUEFORRPROCESSOR2SYMMETRIC_DATAQUEUELED,
	variable: &RPROCESSOR2SYMMETRIC_TASKBODYVAR,
	ex_ctrl_ref: &RPROCESSOR2SYMMETRIC_TASKBODY_EX_CTRL_REF,
};

static RPROCESSOR2SYMMETRIC_TASKBODYVAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is accessed by multiple tasks, but is safe because it is operated exclusively by the mutex object.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
		buffer: unsafe{ &mut *core::ptr::addr_of_mut!(RPROCESSOR2SYMMETRIC_TASKBODYVARARRAY1) },
	buffer_count: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_TASKBODY_EX_CTRL_REF: TECSMutexRef = TECSMutexRef{
	inner: unsafe{MutexRef::from_raw_nonnull(NonZeroI32::new(TECS_RUST_EX_CTRL_1).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY,
};

static mut RPROCESSOR2SYMMETRIC_TASKBODYVARARRAY1: [u32; 8] = [0; 8];

impl<T: SXUartMeasure, U: SDataqueueRs, V: SDataqueueRs> Drop for LockGuardForTTaskbody<'_, T, U, V> {
	fn drop(&mut self){
		self.ex_ctrl_ref.unlock();
	}
}

impl<T: SXUartMeasure, U: SDataqueueRs, V: SDataqueueRs> TTaskbody<T, U, V> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskbody<'_, T, U, V> {
		self.ex_ctrl_ref.lock();
		LockGuardForTTaskbody {
			c_x_uart: self.c_x_uart,
			c_dataqueue: self.c_dataqueue,
			c_dataqueue_led: self.c_dataqueue_led,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
			ex_ctrl_ref: self.ex_ctrl_ref,
		}
	}
}
