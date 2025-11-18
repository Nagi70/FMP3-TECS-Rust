pub struct TXUartInterruptInitializeBody{
}

pub struct ERoutineBodyForTXUartInterruptInitializeBody<'a>{
	pub cell: &'a TXUartInterruptInitializeBody,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTINIBODY: TXUartInterruptInitializeBody = TXUartInterruptInitializeBody {
};

#[link_section = ".rodata"]
pub static EROUTINEBODYFORRPROCESSOR1SYMMETRIC_UARTINIBODY: ERoutineBodyForTXUartInterruptInitializeBody = ERoutineBodyForTXUartInterruptInitializeBody {
	cell: &RPROCESSOR1SYMMETRIC_UARTINIBODY,
};

