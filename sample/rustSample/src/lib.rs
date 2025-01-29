#![no_std]
#![feature(const_option)]
mod kernel_cfg;
mod t_task_rs;
mod t_task_rs_impl;
mod s_task;
mod si_task;
mod s_task_body;
mod si_notification_handler;
mod t_print;
mod t_print_impl;
mod print;

use crate::{t_task_rs::*, s_task_body::*};

#[no_mangle]
pub extern "C" fn rust_start1(_: usize) {
    RPROCESSOR1SYMMETRIC_TASK1_1.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn rust_start2(_: usize) {
    RPROCESSOR2SYMMETRIC_TASK2_1.c_task_body.main();
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
