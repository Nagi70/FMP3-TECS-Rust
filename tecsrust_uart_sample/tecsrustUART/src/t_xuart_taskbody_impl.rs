use crate::{t_xuart_taskbody::*, s_xuart_measure::*, s_task_body::*, si_sio_cbr::*};
use crate::print;
use crate::tecs_print::*;
use itron::abi::*;
use itron::task::*;
use itron::task::State::*;

use itron::time::{duration, Duration, timeout, Timeout};
impl STaskBody for ETaskbodyForTXuartTaskbody<'_>{

	fn main(&'static self) {
		let c_xuart = self.cell.get_cell_ref();

		delay(duration!(s: 1)).expect("delay failed");

		print!("Processor1: Uart task start", );
		delay(duration!(ms: 100)).expect("delay failed");

		c_xuart.open();

		let mut result = true;

		loop{

			result = c_xuart.put_char(&b'N');


			if(result != true)
			{
				print!("uart false",);
			}

			// _ = c_xuart.put_char(&b'\n');

			delay(duration!(s: 1)).expect("delay failed");
		}
	}
}

impl SiSioCbr for EXuartMainForTXuartTaskbody<'_>{

	fn ready_send(&'static self) {
		let c_xuart = self.cell.get_cell_ref();

	}
	fn ready_receive(&'static self) {
		let c_xuart = self.cell.get_cell_ref();
		let mut c: u8 = 0;
		let result = c_xuart.get_char(&mut c);
		if result {
			c_xuart.put_char(&c);
		}
	}
}

