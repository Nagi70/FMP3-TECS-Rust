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
use core::ptr::{write_volatile, read_volatile};

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

const LED_DATA: u32 = 0xE000A040;
const LED_DIRM: u32 = 0xE000A204;
const LED_OEN: u32 = 0xE000A20C;

fn led_setup()
{
    // LED setup
    unsafe{
        let dirm = read_volatile(LED_DIRM as *const u32) | (0x01 << 7);
        let oen = read_volatile(LED_OEN as *const u32) | (0x01 << 7);
        write_volatile(LED_DIRM as *mut u32, dirm);
        write_volatile(LED_OEN as *mut u32, oen);
    }
}

fn light_on()
{
    // LED on
    unsafe{
        let temp = read_volatile(LED_DATA as *const u32) | (0x01 << 7);
        write_volatile(LED_DATA as *mut u32, temp);
    }
}

fn light_off()
{
    // LED off
    unsafe{
    	let temp = read_volatile(LED_DATA as *const u32) & !(0x01 << 7);
    	write_volatile(LED_DATA as *mut u32, temp);
    }
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__led_task(_: usize) {
    print!("Processor1: Rust, LED task", );
    delay(duration!(ms: 1000)).expect("delay failed");

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

    led_setup();

    for i in 0..N{

        unsafe{ 
            _ = loc_cpu();
            start = fch_hrt();
        }

        light_on();
        // light_off();

        unsafe{ 
            end = fch_hrt();
            _ = unl_cpu();
        }

        duration = end - start - overhead;
        print!("%tu,", duration );

        delay(duration!(ms: 10)).expect("delay failed");

        // light_on();
        light_off();

        delay(duration!(ms: 10)).expect("delay failed");
    }
}

const XUART_BASE_ADDRESS: u32 = 0xE0001000;

const XUART_CR_OFFSET: u32 = 0x00;
const XUART_MR_OFFSET: u32 = 0x04;
const XUART_IER_OFFSET: u32 = 0x08;
const XUART_IDR_OFFSET: u32 = 0x0c;
const XUART_ISR_OFFSET: u32 = 0x14;
const XUART_BAUDGEN_OFFSET: u32 = 0x18;
const XUART_RXTOUT_OFFSET: u32 = 0x1c;
const XUART_RXWM_OFFSET: u32 = 0x20;
const XUART_SR_OFFSET: u32 = 0x2c;
const XUART_FIFO_OFFSET: u32 = 0x30;
const XUART_BAUDDIV_OFFSET: u32 = 0x34;

const XUART_CR_STOPPRK: u32 = 0x0100;
const XUART_CR_TX_DIS: u32 = 0x0020;
const XUART_CR_TX_EN: u32 = 0x0010;
const XUART_CR_RX_DIS: u32 = 0x0008;
const XUART_CR_RX_EN: u32 = 0x0004;
const XUART_CR_TXRST: u32 = 0x0002;
const XUART_CR_RXRST: u32 = 0x0001;

const XUART_MR_STOPBIT_1: u32 = 0x0000;
const XUART_MR_PARITY_NONE: u32 = 0x0020;
const XUART_MR_CHARLEN_8: u32 = 0x0000;
const XUART_MR_CLKSEL: u32 = 0x0001;
const XUART_MR_CCLK: u32 = 0x0400;

const XUART_IXR_TXEMPTY: u32 = 0x0008;
const XUART_IXR_RXTRIG: u32 = 0x0001;
const XUART_IXR_ALL: u32 = 0x1fff;

const XUART_SR_TXFULL: u32 = 0x0010;
const XUART_SR_TXEMPTY: u32 = 0x0008;
const XUART_SR_RXEMPTY: u32 = 0x0002;

const XUART_BAUDGEN_115K: u32 = 0x7c;
const XUART_BAUDDIV_115K: u32 = 0x06;

fn uart_setup(){
    unsafe{
    	write_volatile((XUART_BASE_ADDRESS + XUART_IDR_OFFSET) as *mut u32, XUART_IXR_ALL);

    	write_volatile((XUART_BASE_ADDRESS + XUART_ISR_OFFSET) as *mut u32, read_volatile((XUART_BASE_ADDRESS + XUART_ISR_OFFSET) as *const u32));

    	write_volatile((XUART_BASE_ADDRESS + XUART_CR_OFFSET) as *mut u32,
    				   XUART_CR_TXRST | XUART_CR_RXRST | XUART_CR_TX_DIS | XUART_CR_RX_DIS);
        
    	write_volatile((XUART_BASE_ADDRESS + XUART_BAUDGEN_OFFSET) as *mut u32, XUART_BAUDGEN_115K);
    	write_volatile((XUART_BASE_ADDRESS + XUART_BAUDDIV_OFFSET) as *mut u32, XUART_BAUDDIV_115K);

    	write_volatile((XUART_BASE_ADDRESS + XUART_MR_OFFSET) as *mut u32,
    				   XUART_MR_CHARLEN_8 | XUART_MR_PARITY_NONE | XUART_MR_STOPBIT_1);

    	write_volatile((XUART_BASE_ADDRESS + XUART_RXWM_OFFSET) as *mut u32, 0x01);
        
    	write_volatile((XUART_BASE_ADDRESS + XUART_RXTOUT_OFFSET) as *mut u32, 0x10);

    	write_volatile((XUART_BASE_ADDRESS + XUART_CR_OFFSET) as *mut u32,
    				   XUART_CR_TX_EN | XUART_CR_RX_EN | XUART_CR_STOPPRK);
    }
}

fn put_char(c: u8) -> bool
{
    unsafe{
        if((read_volatile((XUART_BASE_ADDRESS + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0x00)
        {
            write_volatile((XUART_BASE_ADDRESS + XUART_FIFO_OFFSET) as *mut u32, c as u32);
            true
        }else{
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor1_symmetric__uart_task(_: usize) {
    print!("Processor1: Rust, Uart task", );
    delay(duration!(ms: 1000)).expect("delay failed");

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

    uart_setup();

    let mut result = true;

    for i in 0..N{

        unsafe{ 
            _ = loc_cpu();
            start = fch_hrt();
        }

        result = put_char(b'N');

        unsafe{ 
            end = fch_hrt();
            _ = unl_cpu();
        }

        duration = end - start - overhead;
        print!("%tu,", duration );

        if(result != true)
        {
            print!("uart false", );
        }

        delay(duration!(ms: 30)).expect("delay failed");
    }
}

#[no_mangle]
pub extern "C" fn tecs_rust_start_r_processor2_symmetric__button_task(_: usize) {
    loop{
    }
}
