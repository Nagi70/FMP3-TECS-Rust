#![no_std]
#![feature(const_option)]

mod tecs_print;
mod kernel_cfg;

use crate::tecs_print::*;
use core::num::NonZeroI32;
use itron::task::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;
use kernel_cfg::*;
use itron::abi::*;
use itron::time::{duration, Duration, timeout, Timeout};
use itron::task::State::*;

unsafe extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

static c_task :TaskRef = unsafe{TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_2_1).unwrap())};

const N :u32 = 1000;

#[no_mangle]
pub unsafe extern "C" fn tecs_rust_start_r_processor1_symmetric__task1_1(_: usize) {

    #[cfg(feature = "get_pri")]
    {
        print!("Evaluate get_pri", );
    }

    delay(duration!(ms: 1000)).expect("delay failed");

    let mut dispatch_time :HrtCnt = 0;
    let mut dispatch_end :HrtCnt = 0;
    let mut overhead :HrtCnt = 0;

    let mut start : HrtCnt = 0;
    let mut end : HrtCnt = 0;

    let mut start_u : u32 = 0;
    let mut start_l : u32 = 0;
    let mut end_u : u32 = 0;
    let mut end_l : u32 = 0;

    unsafe{
        start_u = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
        start_l = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
    }
    for i in 0..N {
        unsafe{
            end_u = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
            end_l = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
        }
    }
    unsafe {
        let cnt64_overhead_start = ((start_u as u64) << 32) | (start_l as u64);
        dispatch_time = cnt64_overhead_start as crate::tecs_print::HrtCnt;

        let cnt64_overhead_end = ((end_u as u64) << 32) | (end_l as u64);
        dispatch_end = cnt64_overhead_end as crate::tecs_print::HrtCnt;
    }

    overhead = (dispatch_end - dispatch_time) / N;

    print!("Overhead: %tu", overhead);

    let mut get_result :Result<Priority, Error<PriorityError>> = Ok(0);

    delay(duration!(ms: 1000)).expect("delay failed");

    for i in 0..N {
        unsafe{
            start_u = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
            start_l = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
        }

        #[cfg(feature = "get_pri")]
        {	get_result = c_task.priority();	}

        unsafe{
            end_u = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
            end_l = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
        }

        unsafe{
            let cnt64_start = ((start_u as u64) << 32) | (start_l as u64);
            start = cnt64_start as crate::tecs_print::HrtCnt;
            let cnt64_end = ((end_u as u64) << 32) | (end_l as u64);
            end = cnt64_end as crate::tecs_print::HrtCnt;
        }

        unsafe{
            let duration = end - start - overhead;
            print!("%tu,", duration);
            delay(duration!(ms: 5)).expect("delay failed");
        }


        #[cfg(all(feature = "get_pri", feature = "debug_task_info"))]
        {
            match get_result {
                Ok(pri) => {
                    print!("get_pri succcess %tu", pri);
                },
                Err(_) => {
                    print!("get_pri error", );
                },
            }
            delay(duration!(ms: 5)).expect("delay failed");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tecs_rust_start_r_processor2_symmetric__task2_1(_: usize) {

    loop{}
}