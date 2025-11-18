use crate::tecs_global::*;
pub struct TMioLed{
	data_0: u32,
	dirm_0: u32,
	oen_0: u32,
}

pub struct ELedForTMioLed {
	pub cell: &'static TMioLed,
}

pub struct LockGuardForTMioLed<'a>{
	pub data_0: &'a u32,
	pub dirm_0: &'a u32,
	pub oen_0: &'a u32,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_LED: TMioLed = TMioLed {
	data_0: 0xE000A040,
	dirm_0: 0xE000A204,
	oen_0: 0xE000A20C,
};

#[unsafe(link_section = ".rodata")]
pub static ELEDFORRPROCESSOR1SYMMETRIC_LED: ELedForTMioLed = ELedForTMioLed {
	cell: &RPROCESSOR1SYMMETRIC_LED,
};

impl TMioLed {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTMioLed {
		LockGuardForTMioLed {
			data_0: &self.data_0,
			dirm_0: &self.dirm_0,
			oen_0: &self.oen_0,
		}
	}
}
