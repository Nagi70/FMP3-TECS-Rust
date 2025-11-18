pub struct TXuartInterruptInitializeBody{
}

pub struct ERoutineBodyForTXuartInterruptInitializeBody<'a>{
	pub cell: &'a TXuartInterruptInitializeBody,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTINIBODY: TXuartInterruptInitializeBody = TXuartInterruptInitializeBody {
};

#[link_section = ".rodata"]
pub static EROUTINEBODYFORRPROCESSOR1SYMMETRIC_UARTINIBODY: ERoutineBodyForTXuartInterruptInitializeBody = ERoutineBodyForTXuartInterruptInitializeBody {
	cell: &RPROCESSOR1SYMMETRIC_UARTINIBODY,
};

