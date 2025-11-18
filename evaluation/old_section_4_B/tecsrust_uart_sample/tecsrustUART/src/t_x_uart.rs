use crate::{si_sio_cbr::*, t_x_uart_taskbody::*};

pub struct TXUart<'a, T>
where
	T: SiSioCbr,
{
	c_x_uart_main: &'a T,
	base_address: u32,
	mode: u32,
	baudgen: u32,
	bauddiv: u32,
}

pub struct EXUartForTXUart<'a>{
	pub cell: &'a TXUart<'a, EXUartMainForTXUartTaskbody<'a>>,
}

pub struct EiHandlerBodyForTXUart<'a>{
	pub cell: &'a TXUart<'a, EXUartMainForTXUartTaskbody<'a>>,
}

pub struct LockGuardForTXUart<'a, T>
where
	T: SiSioCbr,
{
	pub c_x_uart_main: &'a T,
	pub base_address: &'a u32,
	pub mode: &'a u32,
	pub baudgen: &'a u32,
	pub bauddiv: &'a u32,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UART: TXUart<EXUartMainForTXUartTaskbody> = TXUart {
	c_x_uart_main: &EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	base_address: 0xE0001000,
	mode: 0x0020,
	baudgen: 0x007c,
	bauddiv: 0x06,
};

#[link_section = ".rodata"]
pub static EXUARTFORRPROCESSOR1SYMMETRIC_UART: EXUartForTXUart = EXUartForTXUart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

#[link_section = ".rodata"]
pub static EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART: EiHandlerBodyForTXUart = EiHandlerBodyForTXUart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

impl<T: SiSioCbr> TXUart<'_, T> {
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUart<'_, T> {
		LockGuardForTXUart {
			c_x_uart_main: self.c_x_uart_main,
			base_address: &self.base_address,
			mode: &self.mode,
			baudgen: &self.baudgen,
			bauddiv: &self.bauddiv,
		}
	}
}
