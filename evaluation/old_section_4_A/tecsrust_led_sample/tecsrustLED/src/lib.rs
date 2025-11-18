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
mod t_mio_led;
mod t_mio_led_impl;
mod s_led;
mod t_led_taskbody;
mod t_led_taskbody_impl;
mod t_xuart_taskbody;
mod t_xuart_taskbody_impl;
mod s_xuart_measure;
mod t_xuart;
mod t_xuart_impl;
mod t_button_taskbody;
mod t_button_taskbody_impl;
mod s_button;
mod t_button;
mod t_button_impl;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use crate::t_task_rs::*;
use s_task_body::*;

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__led_task(_: usize) {
	RPROCESSOR1SYMMETRIC_LEDTASK.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__uart_task(_: usize) {
	RPROCESSOR1SYMMETRIC_UARTTASK.c_task_body.main();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__button_task(_: usize) {
	RPROCESSOR2SYMMETRIC_BUTTONTASK.c_task_body.main();
}
