#![no_std]
#![feature(const_option)]

mod tecs_print;
mod kernel_cfg;
mod x_can;

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
use core::ptr::{write_volatile, read_volatile};
use x_can::*;

extern "C" {
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

const N :u32 = 1000;

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__can_task(_: usize) {

    print!("CAN task start",);

    x_can_loopback_setup();

    let test_message_id = 1024;
    let frame_data_length = 8;
    let mut tx_frame: [u32; 16] = [0; 16];
    let mut rx_frame: [u32; 16] = [0; 16];

    tx_frame[0] = x_can_create_id_value(test_message_id, 0, 0, 0, 0);
    // print!("x_can_create_id_value",);
    tx_frame[1] = x_can_create_dlc_value(frame_data_length);
    // print!("x_can_create_dlc_value",);

    {
        let frame_data = unsafe {
            core::slice::from_raw_parts_mut(
                tx_frame.as_mut_ptr().add(2) as *mut u8,
                frame_data_length as usize,
            )
        };

        for (index, byte) in frame_data.iter_mut().enumerate() {
            *byte = index as u8;
        }
    }

    let mut dispatch_time :HrtCnt = 0;
    let mut dispatch_end :HrtCnt = 0;
    let mut overhead :HrtCnt = 0;

    unsafe{ dispatch_time = fch_hrt();}
    for i in 0..N {
        unsafe{ dispatch_end = fch_hrt();}
    }

    overhead = (dispatch_end - dispatch_time) / N;

    let mut start :HrtCnt = 0;
    let mut end :HrtCnt = 0;
    let mut duration :HrtCnt = 0;

    for i in 0..N {

        // delay(duration!(s: 1)).expect("delay failed");

        {
            let frame_data = unsafe {
                core::slice::from_raw_parts_mut(
                    tx_frame.as_mut_ptr().add(2) as *mut u8,
                    frame_data_length as usize,
                )
            };
    
            for (index, byte) in frame_data.iter_mut().enumerate() {
                *byte = index as u8;
                // *byte = (i as u8) % core::u8::MAX;
            }
        }

        // for index in 0..4 {
        // 	print!("tx_frame[%tu]: %tX", index, tx_frame[index]);
        // }

        while x_can_is_tx_fifo_full() {}

        // unsafe{ 
        // 	_ = loc_cpu();
        // 	start = fch_hrt();
        // }

        let tx_result = x_can_send(&tx_frame);

        // unsafe{ 
        // 	end = fch_hrt();
        // 	_ = unl_cpu();
        // }

        // duration = end - start - overhead;
        // print!("%tu,", duration );

        if !tx_result {
            print!("failure: x_can_send", );
            loop{}
        }

        while x_can_is_rx_empty() {
            // print!("Wait rx_empty",);
            // let ier = x_can_read_reg(0xE0008000 as *const u32, XCAN_IER_OFFSET);
            // print!("Enabled interrupt: %tX", ier);
            // delay(duration!(ms: 50)).expect("delay failed");
        }
        
        unsafe{ 
            _ = loc_cpu();
            start = fch_hrt();
        }

        let rx_result = x_can_receive(&mut rx_frame);

        unsafe{ 
            end = fch_hrt();
            _ = unl_cpu();
        }

        duration = end - start - overhead;
        print!("%tu,", duration );

        // for index in 0..4 {
        // 	print!("rx_frame[%tu]: %tX", index, rx_frame[index]);
        // }

        if rx_result {
            if rx_frame[0] != x_can_create_id_value(test_message_id, 0, 0, 0, 0) {
                print!("Loop back error: Invalide ID",);
            }
    
            if (rx_frame[1] >> XCAN_DLCR_DLC_SHIFT) != (x_can_create_dlc_value(frame_data_length) >> XCAN_DLCR_DLC_SHIFT) {
                print!("Loop back error: Invalide DLC",);
            }

            let frame_data = unsafe {
                core::slice::from_raw_parts(
                    rx_frame.as_ptr().add(2) as *const u8,
                    frame_data_length as usize,
                )
            };

            for (index, &byte) in frame_data.iter().enumerate() {
                if byte != index as u8 {
                    print!("Loopback error: Invalid data",);
                }
            }
        }else{
            print!("failure: x_can_receive",);
            loop{}
        }
    }

    // print!("CAN task finished",);
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__loop_task(_: usize) {
    loop{
    }
}
