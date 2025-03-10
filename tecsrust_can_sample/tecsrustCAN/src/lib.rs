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
mod t_can_taskbody;
mod t_can_taskbody_impl;
mod s_can_measure;
mod t_can;
mod t_can_impl;
mod t_loop_taskbody;
mod t_loop_taskbody_impl;
mod x_can;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use crate::t_task_rs::*;
use s_task_body::*;

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__can_task(_: usize) {
	RPROCESSOR1SYMMETRIC_CANTASK.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__loop_task(_: usize) {
	RPROCESSOR2SYMMETRIC_LOOPTASK.c_task_body.main();
}
