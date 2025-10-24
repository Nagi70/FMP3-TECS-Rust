use itron::mutex::MutexRef;
use itron::semaphore::SemaphoreRef;
use crate::tecs_ex_ctrl::*;
use core::cell::UnsafeCell;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
pub struct TTaskbody{
	is_exclusive: bool,
	variable: &'static SyncTTaskbodyVar,
	ex_ctrl_ref: &'static (dyn LockManager + Sync + Send),
}

pub struct TTaskbodyVar {
	pub dummy: u32,
}

pub struct SyncTTaskbodyVar {
	unsafe_var: UnsafeCell<TTaskbodyVar>,
}

unsafe impl Sync for SyncTTaskbodyVar {}

pub struct ETaskbodyForTTaskbody {
	pub cell: &'static TTaskbody,
}

pub struct LockGuardForTTaskbody<'a>{
	pub is_exclusive: &'a bool,
	pub var: &'a mut TTaskbodyVar,
	ex_ctrl_ref: &'static (dyn LockManager + Sync + Send),
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_TASKBODY1: TTaskbody = TTaskbody {
	is_exclusive: true,
	variable: &RPROCESSOR1SYMMETRIC_TASKBODY1VAR,
	ex_ctrl_ref: &RPROCESSOR1SYMMETRIC_TASKBODY1_EX_CTRL_REF,
};

static RPROCESSOR1SYMMETRIC_TASKBODY1VAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is accessed by multiple tasks, but is safe because it is operated exclusively by the semaphore object.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
	dummy: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_TASKBODY1_EX_CTRL_REF: TECSSemaphoreRef = TECSSemaphoreRef{
	inner: unsafe{SemaphoreRef::from_raw_nonnull(NonZeroI32::new(TECS_RUST_EX_CTRL_1).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY1: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_TASKBODY1,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_TASKBODY2: TTaskbody = TTaskbody {
	is_exclusive: false,
	variable: &RPROCESSOR1SYMMETRIC_TASKBODY2VAR,
	ex_ctrl_ref: &DUMMY_EX_CTRL_REF,
};

static RPROCESSOR1SYMMETRIC_TASKBODY2VAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
	dummy: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY2: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_TASKBODY2,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_TASKBODY3: TTaskbody = TTaskbody {
	is_exclusive: false,
	variable: &RPROCESSOR1SYMMETRIC_TASKBODY3VAR,
	ex_ctrl_ref: &DUMMY_EX_CTRL_REF,
};

static RPROCESSOR1SYMMETRIC_TASKBODY3VAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
	dummy: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY3: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_TASKBODY3,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_TASKBODY4: TTaskbody = TTaskbody {
	is_exclusive: false,
	variable: &RPROCESSOR1SYMMETRIC_TASKBODY4VAR,
	ex_ctrl_ref: &DUMMY_EX_CTRL_REF,
};

static RPROCESSOR1SYMMETRIC_TASKBODY4VAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
	dummy: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_TASKBODY4: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_TASKBODY4,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_TASKBODY2: TTaskbody = TTaskbody {
	is_exclusive: false,
	variable: &RPROCESSOR2SYMMETRIC_TASKBODY2VAR,
	ex_ctrl_ref: &DUMMY_EX_CTRL_REF,
};

static RPROCESSOR2SYMMETRIC_TASKBODY2VAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
	dummy: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY2: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY2,
};

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_TASKBODY3: TTaskbody = TTaskbody {
	is_exclusive: false,
	variable: &RPROCESSOR2SYMMETRIC_TASKBODY3VAR,
	ex_ctrl_ref: &DUMMY_EX_CTRL_REF,
};

static RPROCESSOR2SYMMETRIC_TASKBODY3VAR: SyncTTaskbodyVar = SyncTTaskbodyVar {
	/// This UnsafeCell is safe because it is only accessed by one task due to the call flow and component structure of TECS.
	unsafe_var: UnsafeCell::new(TTaskbodyVar {
	dummy: 0,
	}),
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_TASKBODY3: ETaskbodyForTTaskbody = ETaskbodyForTTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_TASKBODY3,
};

impl<> Drop for LockGuardForTTaskbody<'_> {
	fn drop(&mut self){
		self.ex_ctrl_ref.unlock();
	}
}

impl TTaskbody {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTaskbody<'_> {
		self.ex_ctrl_ref.lock();
		LockGuardForTTaskbody {
			is_exclusive: &self.is_exclusive,
			var: unsafe{&mut *self.variable.unsafe_var.get()},
			ex_ctrl_ref: self.ex_ctrl_ref,
		}
	}
}
