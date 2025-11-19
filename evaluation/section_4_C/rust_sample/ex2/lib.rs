#![no_std]
#![feature(const_option)]

mod kernel_cfg;
mod tecs_print;

use crate::tecs_print::*;
use core::num::NonZeroI32;
use kernel_cfg::*;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

static IS_EXCLUSIVE1 : bool = true;
static mut DUMMY1 : u32 = 0;
const SEMAPHORE1 : itron::semaphore::SemaphoreRef = unsafe {
    itron::semaphore::SemaphoreRef::from_raw_nonnull(
        NonZeroI32::new(TECS_RUST_EX_CTRL_1).unwrap()
    )
};

#[unsafe(no_mangle)]
pub extern "C" fn start1(_: usize) {
    match SEMAPHORE1.wait(){
        Ok(_) => {},
        Err(e) => {
            match e {
                BadContext => {
                    print!("BadContextError::BadContext", );
                    loop{}
                },
                NotSupported => {
                    loop{}
                },
                BadId => {
                    print!("BadContextError::BadId", );
                    loop{}
                },
                AccessDenied => {
                    print!("BadContextError::AccessDenied", );
                    loop{}
                },
                Released => {
                    print!("BadContextError::Released", );
                    loop{}
                },
                TerminateRequest => {
                    print!("TerminateErrorReason::BadContext", );
                    loop{}
                },
                Deleted => {
                    print!("BadContextError::Deleted", );
                    loop{}
                },
            }
        },
    }
    if IS_EXCLUSIVE1 {
        // 排他制御ありタスクの処理
        unsafe {
            DUMMY1 += 5;
        }
    } else {
        // 排他制御なしタスクの処理
        unsafe {
            DUMMY1 += 10;
        }
    }
    match SEMAPHORE1.signal(){
        Ok(_) => {},
        Err(e) => {
            match e {
                BadContext => {
                    print!("BadContextError::BadContext", );
                    loop{}
                },
                BadId => {
                    print!("BadContextError::BadId", );
                    loop{}
                },
                AccessDenied => {
                    print!("BadContextError::AccessDenied", );
                    loop{}
                },
                QueueOverflow => {
                    print!("BadContextError::QueueOverflow", );
                    loop{}
                },
            }
        },
    }
}

static IS_EXCLUSIVE2 : bool = true;
static mut DUMMY2 : u32 = 0;
const SEMAPHORE2 : itron::semaphore::SemaphoreRef = unsafe {
    itron::semaphore::SemaphoreRef::from_raw_nonnull(
        NonZeroI32::new(TECS_RUST_EX_CTRL_2).unwrap()
    )
};

#[unsafe(no_mangle)]
pub extern "C" fn start2(_: usize) {
    match SEMAPHORE2.wait(){
        Ok(_) => {},
        Err(e) => {
            match e {
                BadContext => {
                    print!("BadContextError::BadContext", );
                    loop{}
                },
                NotSupported => {
                    loop{}
                },
                BadId => {
                    print!("BadContextError::BadId", );
                    loop{}
                },
                AccessDenied => {
                    print!("BadContextError::AccessDenied", );
                    loop{}
                },
                Released => {
                    print!("BadContextError::Released", );
                    loop{}
                },
                TerminateRequest => {
                    print!("TerminateErrorReason::BadContext", );
                    loop{}
                },
                Deleted => {
                    print!("BadContextError::Deleted", );
                    loop{}
                },
            }
        },
    }
    if IS_EXCLUSIVE2 {
        // 排他制御ありタスクの処理
        unsafe {
            DUMMY2 += 5;
        }
    } else {
        // 排他制御なしタスクの処理
        unsafe {
            DUMMY2 += 10;
        }
    }
    match SEMAPHORE2.signal(){
        Ok(_) => {},
        Err(e) => {
            match e {
                BadContext => {
                    print!("BadContextError::BadContext", );
                    loop{}
                },
                BadId => {
                    print!("BadContextError::BadId", );
                    loop{}
                },
                AccessDenied => {
                    print!("BadContextError::AccessDenied", );
                    loop{}
                },
                QueueOverflow => {
                    print!("BadContextError::QueueOverflow", );
                    loop{}
                },
            }
        },
    }
}

static IS_EXCLUSIVE3 : bool = true;
static mut DUMMY3 : u32 = 0;
const SEMAPHORE3 : itron::semaphore::SemaphoreRef = unsafe {
    itron::semaphore::SemaphoreRef::from_raw_nonnull(
        NonZeroI32::new(TECS_RUST_EX_CTRL_3).unwrap()
    )
};

#[unsafe(no_mangle)]
pub extern "C" fn start3(_: usize) {
    match SEMAPHORE3.wait(){
        Ok(_) => {},
        Err(e) => {
            match e {
                BadContext => {
                    print!("BadContextError::BadContext", );
                    loop{}
                },
                NotSupported => {
                    loop{}
                },
                BadId => {
                    print!("BadContextError::BadId", );
                    loop{}
                },
                AccessDenied => {
                    print!("BadContextError::AccessDenied", );
                    loop{}
                },
                Released => {
                    print!("BadContextError::Released", );
                    loop{}
                },
                TerminateRequest => {
                    print!("TerminateErrorReason::BadContext", );
                    loop{}
                },
                Deleted => {
                    print!("BadContextError::Deleted", );
                    loop{}
                },
            }
        },
    }
    if IS_EXCLUSIVE3 {
        // 排他制御ありタスクの処理
        unsafe {
            DUMMY3 += 5;
        }
    } else {
        // 排他制御なしタスクの処理
        unsafe {
            DUMMY3 += 10;
        }
    }
    match SEMAPHORE3.signal(){
        Ok(_) => {},
        Err(e) => {
            match e {
                BadContext => {
                    print!("BadContextError::BadContext", );
                    loop{}
                },
                BadId => {
                    print!("BadContextError::BadId", );
                    loop{}
                },
                AccessDenied => {
                    print!("BadContextError::AccessDenied", );
                    loop{}
                },
                QueueOverflow => {
                    print!("BadContextError::QueueOverflow", );
                    loop{}
                },
            }
        },
    }
}