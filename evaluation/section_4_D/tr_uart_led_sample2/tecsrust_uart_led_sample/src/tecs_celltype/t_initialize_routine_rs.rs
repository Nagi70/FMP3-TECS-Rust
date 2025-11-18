use crate::tecs_global::*;
use crate::tecs_signature::s_routine_body::*;
use crate::tecs_celltype::t_x_uart_interrupt_initialize_body::*;
pub struct TInitializeRoutineRs<T>
where
	T: SRoutineBody + 'static,
{
	pub c_initialize_routine_body: &'static T,
}

pub struct LockGuardForTInitializeRoutineRs<'a, T>
where
	T: SRoutineBody + 'static,
{
	pub c_initialize_routine_body: &'a T,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_UARTINI: TInitializeRoutineRs<ERoutineBodyForTXUartInterruptInitializeBody> = TInitializeRoutineRs {
	c_initialize_routine_body: &EROUTINEBODYFORRPROCESSOR1SYMMETRIC_UARTINIBODY,
};

