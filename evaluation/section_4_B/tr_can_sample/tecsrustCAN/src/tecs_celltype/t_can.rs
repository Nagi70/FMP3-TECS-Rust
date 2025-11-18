use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use itron::semaphore::SemaphoreRef;
pub struct TCan{
	base_address: u32,
	message_id: u32,
	frame_data_length: u32,
	brpr_baud_prescalar: u8,
	btr_sync_jump_width: u8,
	btr_second_timesegment: u8,
	btr_first_timesegment: u8,
	variable: &'static SyncTCanVar,
}

pub struct TCanVar {
	pub var_opt: u32,
}

pub struct SyncTCanVar {
	unsafe_var: UnsafeCell<TCanVar>,
}

unsafe impl Sync for SyncTCanVar {}

pub struct ECanForTCan {
	pub cell: &'static TCan,
}

pub struct LockGuardForTCan<'a>{
	pub base_address: &'a u32,
	pub message_id: &'a u32,
	pub frame_data_length: &'a u32,
	pub brpr_baud_prescalar: &'a u8,
	pub btr_sync_jump_width: &'a u8,
	pub btr_second_timesegment: &'a u8,
	pub btr_first_timesegment: &'a u8,
	pub var: &'a mut TCanVar,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_CAN: TCan = TCan {
	base_address: 0xE0008000,
	message_id: 1024,
	frame_data_length: 8,
	brpr_baud_prescalar: 29,
	btr_sync_jump_width: 3,
	btr_second_timesegment: 2,
	btr_first_timesegment: 15,
	variable: &RPROCESSOR1SYMMETRIC_CANVAR,
};

static RPROCESSOR1SYMMETRIC_CANVAR: SyncTCanVar = SyncTCanVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TCanVar {
	var_opt: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ECANFORRPROCESSOR1SYMMETRIC_CAN: ECanForTCan = ECanForTCan {
	cell: &RPROCESSOR1SYMMETRIC_CAN,
};

impl TCan {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTCan<'_> {
		LockGuardForTCan {
			base_address: &self.base_address,
			message_id: &self.message_id,
			frame_data_length: &self.frame_data_length,
			brpr_baud_prescalar: &self.brpr_baud_prescalar,
			btr_sync_jump_width: &self.btr_sync_jump_width,
			btr_second_timesegment: &self.btr_second_timesegment,
			btr_first_timesegment: &self.btr_first_timesegment,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
		}
	}
}
