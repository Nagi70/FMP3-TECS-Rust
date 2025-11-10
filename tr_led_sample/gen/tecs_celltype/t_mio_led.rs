use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
pub struct TMioLed{
	data_0: u32,
	dirm_0: u32,
	oen_0: u32,
	variable: &'static SyncTMioLedVar,
}

pub struct TMioLedVar {
	pub var_for_opt: u32,
}

pub struct SyncTMioLedVar {
	unsafe_var: UnsafeCell<TMioLedVar>,
}

unsafe impl Sync for SyncTMioLedVar {}

pub struct ELedForTMioLed {
	pub cell: &'static TMioLed,
}

pub struct LockGuardForTMioLed<'a>{
	pub data_0: &'a u32,
	pub dirm_0: &'a u32,
	pub oen_0: &'a u32,
	pub var: &'a mut TMioLedVar,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_MIOLED: TMioLed = TMioLed {
	data_0: 0xE000A040,
	dirm_0: 0xE000A204,
	oen_0: 0xE000A20C,
	variable: &RPROCESSOR1SYMMETRIC_MIOLEDVAR,
};

static RPROCESSOR1SYMMETRIC_MIOLEDVAR: SyncTMioLedVar = SyncTMioLedVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TMioLedVar {
	var_for_opt: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ELEDFORRPROCESSOR1SYMMETRIC_MIOLED: ELedForTMioLed = ELedForTMioLed {
	cell: &RPROCESSOR1SYMMETRIC_MIOLED,
};

impl TMioLed {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTMioLed<'_> {
		LockGuardForTMioLed {
			data_0: &self.data_0,
			dirm_0: &self.dirm_0,
			oen_0: &self.oen_0,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
		}
	}
}
