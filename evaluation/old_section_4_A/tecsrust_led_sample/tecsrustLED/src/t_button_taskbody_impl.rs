use crate::{t_button_taskbody::*, s_task_body::*, s_button::*};

use core::ptr::{write_volatile, read_volatile};
use crate::print;
use crate::tecs_print::*;
use itron::task::*;
use itron::semaphore::SemaphoreRef;
use itron::time::{duration, Duration, timeout, Timeout};
use crate::kernel_cfg::*;
use core::num::NonZeroI32;


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

// const semaphore: SemaphoreRef = unsafe{SemaphoreRef::from_raw_nonnull(NonZeroI32::new(SEMID_1).unwrap())};


impl STaskBody for ETaskbodyForTButtonTaskbody<'_>{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();

		print!("\t\t\tProcessor2: Button task start", );

		// let int_type_1 :u32 = 0xE000A25C;
		// let int_polarity_1 :u32 = 0xE000A260;
		// let int_any_1 :u32 = 0xE000A264;
		// let int_en_1 :u32 = 0xE000A250;
		// let int_stat_1 :u32 = 0xE000A258;
		// let int_dis_1 :u32 = 0xE000A254;
		// let dirm_1 :u32 = 0xE000A244;

		// let gpio = 19;

		// unsafe{
		// 	let mut temp = read_volatile(dirm_1 as *const u32);
		// 	// temp &= !(0x01 << gpio);
		// 	temp |= (0x01 << gpio);
		// 	write_volatile(dirm_1 as *mut u32, temp);
		// }

		// unsafe{
		// 	let mut temp = read_volatile(int_type_1 as *const u32);
		// 	temp |= (0x01 << gpio);
		// 	write_volatile(int_type_1 as *mut u32, temp);
		// }

		// unsafe{
		// 	let mut temp = read_volatile(int_polarity_1 as *const u32);
		// 	temp |= (0x01 << gpio);
		// 	write_volatile(int_polarity_1 as *mut u32, temp);
		// }

		// unsafe{
		// 	let mut temp = read_volatile(int_any_1 as *const u32);
		// 	temp &= !(0x01 << gpio);
		// 	write_volatile(int_any_1 as *mut u32, temp);
		// }

		// unsafe{
		// 	write_volatile(int_en_1 as *mut u32, (0x01 << gpio));
		// }

		// loop{
		// 	if(c_button1.is_pushed())
		// 	{
		// 		print!("\t\t\tButton4 => 1",);
		// 	}else{
		// 		print!("\t\t\tButton4 => 0",);
		// 	}
		// 	delay(duration!(ms: 1000)).expect("delay failed");
			
		// 	unsafe{
		// 		let temp = read_volatile(int_stat_1 as *const u32);
		// 		print!("\t\t\tINT_STATUS_0: %X", temp);
		// 	}
		// }

		// unsafe{
		// 	write_volatile((XUART_BASE_ADDRESS + XUART_IDR_OFFSET) as *mut u32, XUART_IXR_ALL);

		// 	write_volatile((XUART_BASE_ADDRESS + XUART_ISR_OFFSET) as *mut u32, read_volatile((XUART_BASE_ADDRESS + XUART_ISR_OFFSET) as *const u32));

		// 	write_volatile((XUART_BASE_ADDRESS + XUART_CR_OFFSET) as *mut u32,
		// 				   XUART_CR_TXRST | XUART_CR_RXRST | XUART_CR_TX_DIS | XUART_CR_RX_DIS);
			
		// 	write_volatile((XUART_BASE_ADDRESS + XUART_BAUDGEN_OFFSET) as *mut u32, XUART_BAUDGEN_115K);
		// 	write_volatile((XUART_BASE_ADDRESS + XUART_BAUDDIV_OFFSET) as *mut u32, XUART_BAUDDIV_115K);

		// 	write_volatile((XUART_BASE_ADDRESS + XUART_MR_OFFSET) as *mut u32,
		// 				   XUART_MR_CHARLEN_8 | XUART_MR_PARITY_NONE | XUART_MR_STOPBIT_1);

		// 	write_volatile((XUART_BASE_ADDRESS + XUART_RXWM_OFFSET) as *mut u32, 0x01);
			
		// 	write_volatile((XUART_BASE_ADDRESS + XUART_RXTOUT_OFFSET) as *mut u32, 0x10);

		// 	write_volatile((XUART_BASE_ADDRESS + XUART_CR_OFFSET) as *mut u32,
		// 				   XUART_CR_TX_EN | XUART_CR_RX_EN | XUART_CR_STOPPRK);
		// }


		// loop{
		// 	semaphore.wait();
		// 	for _ in 0..2 {
		// 		unsafe{
		// 			if((read_volatile((XUART_BASE_ADDRESS + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0x00)
		// 			{
		// 				write_volatile((XUART_BASE_ADDRESS + XUART_FIFO_OFFSET) as *mut u32, b'\t' as u32);
		// 			}
		// 		}
		// 	}

		// 	unsafe{
		// 		if((read_volatile((XUART_BASE_ADDRESS + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0x00)
		// 		{
		// 			write_volatile((XUART_BASE_ADDRESS + XUART_FIFO_OFFSET) as *mut u32, b'N' as u32);
		// 		}
		// 	}

		// 	unsafe{
		// 		if((read_volatile((XUART_BASE_ADDRESS + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0x00)
		// 		{
		// 			write_volatile((XUART_BASE_ADDRESS + XUART_FIFO_OFFSET) as *mut u32, b'\n' as u32);
		// 		}
		// 	}
		// 	semaphore.signal();
		// 	delay(duration!(ms: 1000)).expect("delay failed");
		// }

	}
}

