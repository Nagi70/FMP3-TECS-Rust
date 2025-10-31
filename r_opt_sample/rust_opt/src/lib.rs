#![no_std]
#![feature(const_option)]

mod kernel_cfg;

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
    SEMAPHORE1.wait().expect("semaphore wait failed");
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
    SEMAPHORE1.signal().expect("semaphore signal failed");
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
    SEMAPHORE2.wait().expect("semaphore wait failed");
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
    SEMAPHORE2.signal().expect("semaphore signal failed");
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
    SEMAPHORE3.wait().expect("semaphore wait failed");
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
    SEMAPHORE3.signal().expect("semaphore signal failed");
}

static IS_EXCLUSIVE4 : bool = true;
static mut DUMMY4 : u32 = 0;
const SEMAPHORE4 : itron::semaphore::SemaphoreRef = unsafe {
    itron::semaphore::SemaphoreRef::from_raw_nonnull(
        NonZeroI32::new(TECS_RUST_EX_CTRL_4).unwrap()
    )
};

#[unsafe(no_mangle)]
pub extern "C" fn start4(_: usize) {
    SEMAPHORE4.wait().expect("semaphore wait failed");
    if IS_EXCLUSIVE4 {
        // 排他制御ありタスクの処理
        unsafe {
            DUMMY4 += 5;
        }
    } else {
        // 排他制御なしタスクの処理
        unsafe {
            DUMMY4 += 10;
        }
    }
    SEMAPHORE4.signal().expect("semaphore signal failed");
}

static IS_EXCLUSIVE5 : bool = true;
static mut DUMMY5 : u32 = 0;
const SEMAPHORE5 : itron::semaphore::SemaphoreRef = unsafe {
    itron::semaphore::SemaphoreRef::from_raw_nonnull(
        NonZeroI32::new(TECS_RUST_EX_CTRL_5).unwrap()
    )
};

#[unsafe(no_mangle)]
pub extern "C" fn start5(_: usize) {
    SEMAPHORE5.wait().expect("semaphore wait failed");
    if IS_EXCLUSIVE5 {
        // 排他制御ありタスクの処理
        unsafe {
            DUMMY5 += 5;
        }
    } else {
        // 排他制御なしタスクの処理
        unsafe {
            DUMMY5 += 10;
        }
    }
    SEMAPHORE5.signal().expect("semaphore signal failed");
}

static IS_EXCLUSIVE6 : bool = true;
static mut DUMMY6 : u32 = 0;
const SEMAPHORE6 : itron::semaphore::SemaphoreRef = unsafe {
    itron::semaphore::SemaphoreRef::from_raw_nonnull(
        NonZeroI32::new(TECS_RUST_EX_CTRL_6).unwrap()
    )
};

#[unsafe(no_mangle)]
pub extern "C" fn start6(_: usize) {
    SEMAPHORE6.wait().expect("semaphore wait failed");
    if IS_EXCLUSIVE6 {
        // 排他制御ありタスクの処理
        unsafe {
            DUMMY6 += 5;
        }
    } else {
        // 排他制御なしタスクの処理
        unsafe {
            DUMMY6 += 10;
        }
    }
    SEMAPHORE6.signal().expect("semaphore signal failed");
}
