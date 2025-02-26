pub struct TXuart{
	base_address: u32,
	mode: u32,
	baudgen: u32,
	bauddiv: u32,
}

pub struct EXuartForTXuart<'a>{
	pub cell: &'a TXuart,
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

impl<> TXuart<> {
	pub fn get_cell_ref(&'static self) -> (&'static u32, &'static u32, &'static u32, &'static u32) {
		(
			&self.base_address,
			&self.mode,
			&self.baudgen,
			&self.bauddiv
		)
	}
}
