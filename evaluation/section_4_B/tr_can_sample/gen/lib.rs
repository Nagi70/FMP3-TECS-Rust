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
pub extern "C" fn tecs_rust_start_r_processor1_symmetric_can_task(_: usize) {
	RPROCESSOR1SYMMETRIC_CANTASK.c_task_body.main();
}

#[unsafe(no_mangle)]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric_loop_task(_: usize) {
	RPROCESSOR2SYMMETRIC_LOOPTASK.c_task_body.main();
}
