use crate::{s_routine_body::*, t_x_uart_interrupt_initialize_body::*};

pub struct TInitializeRoutineRs<'a, T>
where
	T: SRoutineBody,
{
	pub c_initialize_routine_body: &'a T,
}

pub struct LockGuardForTInitializeRoutineRs<'a, T>
where
	T: SRoutineBody,
{
	pub c_initialize_routine_body: &'a T,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_UARTINI: TInitializeRoutineRs<ERoutineBodyForTXUartInterruptInitializeBody> = TInitializeRoutineRs {
	c_initialize_routine_body: &EROUTINEBODYFORRPROCESSOR1SYMMETRIC_UARTINIBODY,
};

