#![no_std]
#![feature(const_option)]
mod kernel_cfg;
mod tecs_ex_ctrl;
mod tecs_print;
mod t_task_rs;
mod t_task_rs_impl;
mod s_task_rs;
mod si_task;
mod s_task_body;
mod si_notification_handler;
mod t_dataqueue_rs;
mod t_dataqueue_rs_impl;
mod s_dataqueue_rs;
mod si_dataqueue_rs;
mod t_isr_rs;
mod si_handler_body;
mod t_initialize_routine_rs;
mod s_routine_body;
mod t_x_uart;
mod t_x_uart_impl;
mod si_sio_cbr;
mod s_x_uart_measure;
mod t_x_uart_taskbody;
mod t_x_uart_taskbody_impl;
mod t_x_uart_interrupt_initialize_body;
mod t_x_uart_interrupt_initialize_body_impl;
mod t_taskbody;
mod t_taskbody_impl;
mod t_xuart;
mod t_xuart_impl;
mod s_xuart_measure;
mod t_xuart_taskbody;
mod t_xuart_taskbody_impl;
mod t_xuart_interrupt_initialize_body;
mod t_xuart_interrupt_initialize_body_impl;
mod xuart;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use crate::t_task_rs::*;
use s_task_body::*;

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric_task1(_: usize) {
	RPROCESSOR1SYMMETRIC_TASK1.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric_task2(_: usize) {
	RPROCESSOR2SYMMETRIC_TASK2.c_task_body.main();
}

use crate::t_isr_rs::*;
use si_handler_body::*;

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric_uart_isr(_: usize) {
	RPROCESSOR1SYMMETRIC_UARTISR.ci_isr_body.main();
}

use crate::t_initialize_routine_rs::*;
use s_routine_body::*;

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric_uart_ini(_: usize) {
	RPROCESSOR1SYMMETRIC_UARTINI.c_initialize_routine_body.main();
}
