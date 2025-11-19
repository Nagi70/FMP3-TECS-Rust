use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use itron::semaphore::SemaphoreRef;
pub struct TXUart{
	base_address: u32,
	mode: u32,
	baudgen: u32,
	bauddiv: u32,
	variable: &'static SyncTXUartVar,
}

pub struct TXUartVar {
	pub count: u32,
}

pub struct SyncTXUartVar {
	unsafe_var: UnsafeCell<TXUartVar>,
}

unsafe impl Sync for SyncTXUartVar {}

pub struct EXUartForTXUart {
	pub cell: &'static TXUart,
}

pub struct LockGuardForTXUart<'a>{
	pub base_address: &'a u32,
	pub mode: &'a u32,
	pub baudgen: &'a u32,
	pub bauddiv: &'a u32,
	pub var: &'a mut TXUartVar,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_UART: TXUart = TXUart {
	base_address: 0xE0001000,
	mode: 0x0020,
	baudgen: 0x007c,
	bauddiv: 0x06,
	variable: &RPROCESSOR1SYMMETRIC_UARTVAR,
};

static RPROCESSOR1SYMMETRIC_UARTVAR: SyncTXUartVar = SyncTXUartVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TXUartVar {
	count: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static EXUARTFORRPROCESSOR1SYMMETRIC_UART: EXUartForTXUart = EXUartForTXUart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

impl TXUart {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUart<'_> {
		LockGuardForTXUart {
			base_address: &self.base_address,
			mode: &self.mode,
			baudgen: &self.baudgen,
			bauddiv: &self.bauddiv,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
		}
	}
}
