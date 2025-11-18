use crate::{t_xuart_taskbody::*, s_xuart_measure::*, s_task_body::*};
use crate::print;
use crate::tecs_print::*;
use itron::abi::*;
use itron::task::*;
use itron::task::State::*;

use itron::time::{duration, Duration, timeout, Timeout};

extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

const N: u32 = 1000;

impl STaskBody for ETaskbodyForTXuartTaskbody<'_>{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();

		delay(duration!(s: 1)).expect("delay failed");

		print!("Processor1: Uart task start", );

		lg.c_xuart.open();

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

		let mut result = true;

		for i in 0..N{
			unsafe{ 
				_ = loc_cpu();
				start = fch_hrt();
			}

			result = lg.c_xuart.put_char(&b'N');

			unsafe{ 
				end = fch_hrt();
				_ = unl_cpu();
			}

			duration = end - start - overhead;
			print!("%tu,", duration );

			if(result != true)
			{
				print!("uart false",);
			}

			delay(duration!(ms: 50)).expect("delay failed");
		}
	}
}

