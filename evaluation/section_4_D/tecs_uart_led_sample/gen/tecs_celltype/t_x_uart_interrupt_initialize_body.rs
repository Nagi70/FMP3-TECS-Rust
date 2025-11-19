use crate::tecs_global::*;
pub struct TXUartInterruptInitializeBody{
}

pub struct ERoutineBodyForTXUartInterruptInitializeBody {
	pub cell: &'static TXUartInterruptInitializeBody,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_UARTINIBODY: TXUartInterruptInitializeBody = TXUartInterruptInitializeBody {
};

#[unsafe(link_section = ".rodata")]
pub static EROUTINEBODYFORRPROCESSOR1SYMMETRIC_UARTINIBODY: ERoutineBodyForTXUartInterruptInitializeBody = ERoutineBodyForTXUartInterruptInitializeBody {
	cell: &RPROCESSOR1SYMMETRIC_UARTINIBODY,
};

