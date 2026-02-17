use crate::tecs_global::*;
pub trait TMioLedConfig: 'static {
    const DATA_0: u32;
    const DIRM_0: u32;
    const OEN_0: u32;
}

pub struct TMioLedData0<CONFIG: TMioLedConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TMioLedConfig> core::ops::Deref for TMioLedData0<CONFIG> {
    type Target = u32;
    #[inline(always)]
    fn deref(&self) -> &u32 {
        &CONFIG::DATA_0
    }
}

pub struct TMioLedDirm0<CONFIG: TMioLedConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TMioLedConfig> core::ops::Deref for TMioLedDirm0<CONFIG> {
    type Target = u32;
    #[inline(always)]
    fn deref(&self) -> &u32 {
        &CONFIG::DIRM_0
    }
}

pub struct TMioLedOen0<CONFIG: TMioLedConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TMioLedConfig> core::ops::Deref for TMioLedOen0<CONFIG> {
    type Target = u32;
    #[inline(always)]
    fn deref(&self) -> &u32 {
        &CONFIG::OEN_0
    }
}

// Instance Configurations
pub struct ConfigLed;
impl TMioLedConfig for ConfigLed {
    const DATA_0: u32 = 0xE000A040;
    const DIRM_0: u32 = 0xE000A204;
    const OEN_0: u32 = 0xE000A20C;
}

pub struct TMioLed<CONFIG>{
	_phantom: core::marker::PhantomData<CONFIG>,
}

pub struct ELedForTMioLed<CONFIG: TMioLedConfig> {
	pub cell: &'static TMioLed<CONFIG>,
}

pub struct LockGuardForTMioLed<CONFIG: TMioLedConfig>{
	pub data_0: TMioLedData0<CONFIG>,
	pub dirm_0: TMioLedDirm0<CONFIG>,
	pub oen_0: TMioLedOen0<CONFIG>,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_LED: TMioLed<ConfigLed> = TMioLed {
	_phantom: core::marker::PhantomData,
};

#[unsafe(link_section = ".rodata")]
pub static ELEDFORRPROCESSOR1SYMMETRIC_LED: ELedForTMioLed<ConfigLed> = ELedForTMioLed {
	cell: &RPROCESSOR1SYMMETRIC_LED,
};

impl<CONFIG: TMioLedConfig> TMioLed<CONFIG> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTMioLed<CONFIG> {
		LockGuardForTMioLed {
			data_0: TMioLedData0(core::marker::PhantomData),
			dirm_0: TMioLedDirm0(core::marker::PhantomData),
			oen_0: TMioLedOen0(core::marker::PhantomData),
		}
	}
}
