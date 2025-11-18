use crate::{t_taskbody::*, s_task_body::*, s_xuart_measure::*, s_dataqueue_rs::*};
use crate::print;
use itron::time::{duration, Duration, timeout, Timeout};
use itron::task::*;


impl STaskBody for ETaskbodyForTTaskbody<'_>{

	fn main(&'static self) {
		let lg = self.cell.get_cell_ref();

		delay(duration!(s: 1)).expect("delay failed");

		loop{
			let mut data = lg.c_dataqueue.receive();
			match data {
				Ok(data) => {
					lg.c_xuart.put_char(&(data as u8));
				}
				Err(e) => {
					lg.c_xuart.put_char(&b'E');
				}
			}
		}
	}
}

