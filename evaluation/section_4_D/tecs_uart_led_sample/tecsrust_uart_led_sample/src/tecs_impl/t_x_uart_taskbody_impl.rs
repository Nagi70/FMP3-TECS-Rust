use crate::tecs_global::*;
use crate::tecs_celltype::t_x_uart_taskbody::*;
use crate::tecs_signature::{s_x_uart_measure::*, s_led::*, si_dataqueue_rs::*, s_dataqueue_rs::*, s_task_body::*, si_sio_cbr::*};

use crate::print;
use crate::tecs_print::*;
use itron::abi::*;
use itron::task::*;
use itron::task::State::*;
use itron::time::{duration, Duration, timeout, Timeout};

impl STaskBody for ETaskbodyForTXUartTaskbody{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();
		
		delay(duration!(s: 1)).expect("delay failed");

		// print!("Processor1: Uart task start", );
		// delay(duration!(ms: 100)).expect("delay failed");

		lg.c_x_uart.open();
		lg.c_led.set_up();

		loop{
			let mut data_led = lg.c_dataqueue_led.receive();
			match data_led {
				Ok(data) => {
					for _ in 0..data {
						lg.c_led.light_on();
						delay(duration!(ms: 50)).expect("delay failed");
						lg.c_led.light_off();
						delay(duration!(ms: 50)).expect("delay failed");
					}
				}
				Err(e) => {
					lg.c_x_uart.put_char(&b'E');
				}
			}
		}

		// let mut result = true;

		// loop{

		// 	result = lg.c_x_uart.put_char(&b'N');


		// 	if(result != true)
		// 	{
		// 		print!("uart false",);
		// 	}

		// 	// _ = c_x_uart.put_char(&b'\n');

		// 	delay(duration!(s: 1)).expect("delay failed");
		// }
	}
}

impl SiSioCbr for EXUartMainForTXUartTaskbody{

	fn ready_send(&self) {
		let lg = self.cell.get_cell_ref();

	}
	fn ready_receive(&self) {
		let lg = self.cell.get_cell_ref();
		let mut c: u8 = 0;
		let result = lg.c_x_uart.get_char(&mut c);
		if result {
			// c_x_uart.put_char(&c);
			let data: itron::dataqueue::DataElement = c.into();
			lg.c_dataqueue.send_force(&data);
		}
	}
}

