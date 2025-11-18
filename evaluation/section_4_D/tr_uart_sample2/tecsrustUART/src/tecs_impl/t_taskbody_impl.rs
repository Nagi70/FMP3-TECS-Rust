use crate::tecs_global::*;
use crate::tecs_celltype::t_taskbody::*;
use crate::tecs_signature::{s_task_body::*, s_x_uart_measure::*, s_dataqueue_rs::*};

use crate::print;
use itron::time::{duration, Duration, timeout, Timeout};
use itron::task::*;

impl STaskBody for ETaskbodyForTTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

		delay(duration!(s: 1)).expect("delay failed");

		loop{
			let mut data = lg.c_dataqueue.receive();
			match data {
				Ok(data) => {
					lg.c_x_uart.put_char(&(data as u8));
				}
				Err(e) => {
					lg.c_x_uart.put_char(&b'E');
				}
			}
		}
	}
}

