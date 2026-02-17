use crate::tecs_global::*;
use crate::tecs_signature::si_sio_cbr::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
pub trait TXUartConfig: 'static {
    const BASE_ADDRESS: u32;
    const MODE: u32;
    const BAUDGEN: u32;
    const BAUDDIV: u32;
}

pub struct TXUartBaseAddress<CONFIG: TXUartConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TXUartConfig> core::ops::Deref for TXUartBaseAddress<CONFIG> {
    type Target = u32;
    #[inline(always)]
    fn deref(&self) -> &u32 {
        &CONFIG::BASE_ADDRESS
    }
}

pub struct TXUartMode<CONFIG: TXUartConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TXUartConfig> core::ops::Deref for TXUartMode<CONFIG> {
    type Target = u32;
    #[inline(always)]
    fn deref(&self) -> &u32 {
        &CONFIG::MODE
    }
}

pub struct TXUartBaudgen<CONFIG: TXUartConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TXUartConfig> core::ops::Deref for TXUartBaudgen<CONFIG> {
    type Target = u32;
    #[inline(always)]
    fn deref(&self) -> &u32 {
        &CONFIG::BAUDGEN
    }
}

pub struct TXUartBauddiv<CONFIG: TXUartConfig>(core::marker::PhantomData<CONFIG>);
impl<CONFIG: TXUartConfig> core::ops::Deref for TXUartBauddiv<CONFIG> {
    type Target = u32;
    #[inline(always)]
    fn deref(&self) -> &u32 {
        &CONFIG::BAUDDIV
    }
}

// Instance Configurations
pub struct ConfigUart;
impl TXUartConfig for ConfigUart {
    const BASE_ADDRESS: u32 = 0xE0001000;
    const MODE: u32 = 0x0020;
    const BAUDGEN: u32 = 0x007c;
    const BAUDDIV: u32 = 0x06;
}

pub struct TXUart<CONFIG>
where
	CONFIG: TXUartConfig,
{
	c_x_uart_main: &'static EXUartMainForTXUartTaskbody,
	_phantom: core::marker::PhantomData<CONFIG>,
}

pub struct EXUartForTXUart<CONFIG: TXUartConfig> {
	pub cell: &'static TXUart<CONFIG>,
}

pub struct EiHandlerBodyForTXUart<CONFIG: TXUartConfig> {
	pub cell: &'static TXUart<CONFIG>,
}

pub struct LockGuardForTXUart<'a, CONFIG: TXUartConfig>
where
	CONFIG: TXUartConfig,
{
	pub c_x_uart_main: &'a EXUartMainForTXUartTaskbody,
	pub base_address: TXUartBaseAddress<CONFIG>,
	pub mode: TXUartMode<CONFIG>,
	pub baudgen: TXUartBaudgen<CONFIG>,
	pub bauddiv: TXUartBauddiv<CONFIG>,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_UART: TXUart<ConfigUart> = TXUart {
	c_x_uart_main: &EXUARTMAINFORRPROCESSOR1SYMMETRIC_UARTTASKBODY,
	_phantom: core::marker::PhantomData,
};

#[unsafe(link_section = ".rodata")]
pub static EXUARTFORRPROCESSOR1SYMMETRIC_UART: EXUartForTXUart<ConfigUart> = EXUartForTXUart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

#[unsafe(link_section = ".rodata")]
pub static EIHANDLERBODYFORRPROCESSOR1SYMMETRIC_UART: EiHandlerBodyForTXUart<ConfigUart> = EiHandlerBodyForTXUart {
	cell: &RPROCESSOR1SYMMETRIC_UART,
};

impl<CONFIG: TXUartConfig> TXUart<CONFIG> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTXUart<CONFIG> {
		LockGuardForTXUart {
			c_x_uart_main: self.c_x_uart_main,
			base_address: TXUartBaseAddress(core::marker::PhantomData),
			mode: TXUartMode(core::marker::PhantomData),
			baudgen: TXUartBaudgen(core::marker::PhantomData),
			bauddiv: TXUartBauddiv(core::marker::PhantomData),
		}
	}
}
