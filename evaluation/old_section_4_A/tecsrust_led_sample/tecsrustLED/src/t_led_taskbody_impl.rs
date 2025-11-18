use crate::{t_led_taskbody::*, s_led::*, s_task_body::*};

use itron::abi::*;
use crate::print;
use crate::tecs_print::*;
use itron::task::*;
use itron::time::{duration, Duration, timeout, Timeout};

use crate::kernel_cfg::*;
use itron::semaphore::SemaphoreRef;
use core::num::NonZeroI32;

extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

const N: u32 = 1000;

impl STaskBody for ETaskbodyForTLedTaskbody<'_>{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();
		print!("Processor1: LED task start", );

		print!("LED setup", );
		lg.c_led.set_up();
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

		for i in 0..N{
			unsafe{ 
				_ = loc_cpu();
				start = fch_hrt();
			}

			lg.c_led.light_on();
			// lg.c_led.light_off();

			unsafe{ 
				end = fch_hrt();
				_ = unl_cpu();
			}

			duration = end - start - overhead;
			print!("%tu,", duration );

			delay(duration!(ms: 10)).expect("delay failed");

			// lg.c_led.light_on();
			lg.c_led.light_off();

			delay(duration!(ms: 10)).expect("delay failed");
		}
	}
}

