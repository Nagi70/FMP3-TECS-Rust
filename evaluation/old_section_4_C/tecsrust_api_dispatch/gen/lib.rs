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
mod t_semaphore_rs;
mod t_semaphore_rs_impl;
mod s_semaphore_rs;
mod si_semaphore;
mod t_measure;
mod t_measure_impl;
mod t_loop_taskbody;
mod t_loop_taskbody_impl;
mod t_stop_taskbody;
mod t_stop_taskbody_impl;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use crate::t_task_rs::*;
use s_task_body::*;

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__task1_1(_: usize) {
	RPROCESSOR1SYMMETRIC_TASK1_1.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor_all_mig__taskmig(_: usize) {
	RPROCESSORALLMIG_TASKMIG.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__task2_1(_: usize) {
	RPROCESSOR2SYMMETRIC_TASK2_1.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__task2_2(_: usize) {
	RPROCESSOR2SYMMETRIC_TASK2_2.c_task_body.main();
}
