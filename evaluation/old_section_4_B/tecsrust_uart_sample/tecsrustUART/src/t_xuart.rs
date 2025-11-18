use crate::{si_sio_cbr::*, t_xuart_taskbody::*};

pub struct TXuart<'a, T>
where
	T: SiSioCbr,
{
	c_xuart_main: &'a T,
	base_address: u32,
	mode: u32,
	baudgen: u32,
	bauddiv: u32,
}

pub struct EXuartForTXuart<'a>{
	pub cell: &'a TXuart<'a, EXuartMainForTXuartTaskbody<'a>>,
}

pub struct EiHandlerBodyForTXuart<'a>{
	pub cell: &'a TXuart<'a, EXuartMainForTXuartTaskbody<'a>>,
}

pub struct LockGuardForTXuart<'a, T>
where
	T: SiSioCbr,
{
	pub c_xuart_main: &'a T,
	pub base_address: &'a u32,
	pub mode: &'a u32,
	pub baudgen: &'a u32,
	pub bauddiv: &'a u32,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UART: TXuart<EXuartMainForTXuartTaskbody> = TXuart {
	c_xuart_main: &EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	base_address: 0xE0001000,
	mode: 0x0020,
	baudgen: 0x007c,
	bauddiv: 0x06,
};

#[link_section = ".rodata"]
pub static EXUARTFORRPROCESSOR1SYMMETRIC_UART: EXuartForTXuart = EXuartForTXuart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

#[link_section = ".rodata"]
pub static EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART: EiHandlerBodyForTXuart = EiHandlerBodyForTXuart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

impl<T: SiSioCbr> TXuart<'_, T> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXuart<'_, T> {
		LockGuardForTXuart {
			c_xuart_main: self.c_xuart_main,
			base_address: &self.base_address,
			mode: &self.mode,
			baudgen: &self.baudgen,
			bauddiv: &self.bauddiv,
		}
	}
}
