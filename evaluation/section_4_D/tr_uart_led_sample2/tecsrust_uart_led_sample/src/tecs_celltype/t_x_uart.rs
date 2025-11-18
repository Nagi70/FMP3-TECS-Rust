use crate::tecs_global::*;
use crate::tecs_signature::si_sio_cbr::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
pub struct TXUart<T>
where
	T: SiSioCbr + 'static,
{
	c_x_uart_main: &'static T,
	base_address: u32,
	mode: u32,
	baudgen: u32,
	bauddiv: u32,
}

pub struct EXUartForTXUart {
	pub cell: &'static TXUart<EXUartMainForTXUartTaskbody>,
}

pub struct EiHandlerBodyForTXUart {
	pub cell: &'static TXUart<EXUartMainForTXUartTaskbody>,
}

pub struct LockGuardForTXUart<'a, T>
where
	T: SiSioCbr + 'static,
{
	pub c_x_uart_main: &'a T,
	pub base_address: &'a u32,
	pub mode: &'a u32,
	pub baudgen: &'a u32,
	pub bauddiv: &'a u32,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_UART: TXUart<EXUartMainForTXUartTaskbody> = TXUart {
	c_x_uart_main: &EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	base_address: 0xE0001000,
	mode: 0x0020,
	baudgen: 0x007c,
	bauddiv: 0x06,
};

#[unsafe(link_section = ".rodata")]
pub static EXUARTFORRPROCESSOR1SYMMETRIC_UART: EXUartForTXUart = EXUartForTXUart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

#[unsafe(link_section = ".rodata")]
pub static EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART: EiHandlerBodyForTXUart = EiHandlerBodyForTXUart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

impl<T: SiSioCbr> TXUart<T> {
	#[inline]
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
