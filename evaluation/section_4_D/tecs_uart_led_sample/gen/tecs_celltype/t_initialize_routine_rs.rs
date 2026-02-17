use crate::tecs_global::*;
use crate::tecs_signature::s_routine_body::*;
use crate::tecs_celltype::t_x_uart_interrupt_initialize_body::*;
pub struct TInitializeRoutineRs{
	pub c_initialize_routine_body: &'static ERoutineBodyForTXUartInterruptInitializeBody,
}

pub struct LockGuardForTInitializeRoutineRs<'a>{
	pub c_initialize_routine_body: &'a ERoutineBodyForTXUartInterruptInitializeBody,
}

#[unsafe(link_section = ".rodata")]
pub static RPROCESSOR1SYMMETRIC_UARTINI: TInitializeRoutineRs = TInitializeRoutineRs {
	c_initialize_routine_body: &EROUTINEBODYFORRPROCESSOR1SYMMETRIC_UARTINIBODY,
};

