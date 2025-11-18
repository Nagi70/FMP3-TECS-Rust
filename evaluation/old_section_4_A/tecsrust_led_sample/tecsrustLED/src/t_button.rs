pub struct TButton{
	data_1_ro: u32,
	gpio_offset: u32,
}

pub struct EButtonForTButton<'a>{
	pub cell: &'a TButton,
}

pub struct LockGuardForTButton<'a>{
	pub data_1_ro: &'a u32,
	pub gpio_offset: &'a u32,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_BUTTON4: TButton = TButton {
	data_1_ro: 0xE000A064,
	gpio_offset: 19,
};

#[link_section = ".rodata"]
pub static EBUTTONFORRPROCESSOR2SYMMETRIC_BUTTON4: EButtonForTButton = EButtonForTButton {
	cell: &RPROCESSOR2SYMMETRIC_BUTTON4,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_BUTTON5: TButton = TButton {
	data_1_ro: 0xE000A064,
	gpio_offset: 20,
};

#[link_section = ".rodata"]
pub static EBUTTONFORRPROCESSOR2SYMMETRIC_BUTTON5: EButtonForTButton = EButtonForTButton {
	cell: &RPROCESSOR2SYMMETRIC_BUTTON5,
};

impl<> TButton {
	pub fn get_cell_ref(&'static self) -> LockGuardForTButton {
		LockGuardForTButton {
			data_1_ro: &self.data_1_ro,
			gpio_offset: &self.gpio_offset,
		}
	}
}
