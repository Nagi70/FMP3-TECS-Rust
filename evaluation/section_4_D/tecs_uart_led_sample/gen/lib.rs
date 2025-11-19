#![no_std]
#![feature(const_option)]
mod kernel_cfg;
mod tecs_ex_ctrl;
mod tecs_print;
mod tecs_celltype;
mod tecs_signature;
mod tecs_impl;
mod tecs_global;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
	loop {}
}

use tecs_celltype::t_task_rs::*;
use tecs_signature::s_task_body::*;

#[unsafe(no_mangle)]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric_uart_task(_: usize) {
	RPROCESSOR1SYMMETRIC_UARTTASK.c_task_body.main();
}

#[unsafe(no_mangle)]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric_button_task(_: usize) {
	RPROCESSOR2SYMMETRIC_BUTTONTASK.c_task_body.main();
}
use tecs_celltype::t_isr_rs::*;
use tecs_signature::si_handler_body::*;

#[unsafe(no_mangle)]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric_uart_isr(_: usize) {
	RPROCESSOR1SYMMETRIC_UARTISR.ci_isr_body.main();
}
use tecs_celltype::t_initialize_routine_rs::*;
use tecs_signature::s_routine_body::*;

#[unsafe(no_mangle)]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric_uart_ini(_: usize) {
	RPROCESSOR1SYMMETRIC_UARTINI.c_initialize_routine_body.main();
}
