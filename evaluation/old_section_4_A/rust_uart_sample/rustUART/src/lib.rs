#![no_std]
#![feature(const_option)]

mod tecs_print;
mod kernel_cfg;
mod xuart;

use crate::tecs_print::*;
use core::num::NonZeroI32;
use itron::task::*;
use kernel_cfg::*;
use itron::time::{duration, Duration, timeout, Timeout};
use xuart::*;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

pub static c_dataqueue: itron::dataqueue::DataqueueRef = unsafe{itron::dataqueue::DataqueueRef::from_raw_nonnull(NonZeroI32::new(DTQID_UART).unwrap())};

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__uart_task(_: usize) {
    delay(duration!(s: 1)).expect("delay failed");

    // print!("Processor1: Uart task start", );
    // delay(duration!(ms: 100)).expect("delay failed");

    let c_xuart = xuart::Xuart::open(0xE0001000);
    c_xuart.set_up(0x0020, 0x007c, 0x06);

    // let mut result = true;

    // loop{

    //     result = c_xuart.put_char(b'N');


    //     if(result != true)
    //     {
    //         print!("uart false",);
    //     }

    //     // _ = c_xuart.put_char(&b'\n');

    //     delay(duration!(s: 1)).expect("delay failed");
    // }
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__button_task(_: usize) {
    delay(duration!(s: 1)).expect("delay failed");

    let c_xuart = xuart::Xuart::open(0xE0001000);

    loop{
        let mut data = c_dataqueue.recv();
        match data {
            Ok(data) => {
                c_xuart.put_char(data as u8);
            }
            Err(e) => {
                c_xuart.put_char(b'E');
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__uart_isr(_: usize) {
    let c_xuart = xuart::Xuart::open(0xE0001000);
    c_xuart.handler();
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__uart_ini(_: usize) {
}



