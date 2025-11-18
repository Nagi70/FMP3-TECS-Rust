pub struct TXuart{
	base_address: u32,
	mode: u32,
	baudgen: u32,
	bauddiv: u32,
}

pub struct EXuartForTXuart<'a>{
	pub cell: &'a TXuart,
}

pub struct LockGuardForTXuart<'a>{
	pub base_address: &'a u32,
	pub mode: &'a u32,
	pub baudgen: &'a u32,
	pub bauddiv: &'a u32,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UART: TXuart = TXuart {
	base_address: 0xE0001000,
	mode: 0x0020,
	baudgen: 0x007c,
	bauddiv: 0x06,
};

#[link_section = ".rodata"]
pub static EXUARTFORRPROCESSOR1SYMMETRIC_UART: EXuartForTXuart = EXuartForTXuart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

impl<> TXuart {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXuart {
		LockGuardForTXuart {
			base_address: &self.base_address,
			mode: &self.mode,
			baudgen: &self.baudgen,
			bauddiv: &self.bauddiv,
		}
	}
}
