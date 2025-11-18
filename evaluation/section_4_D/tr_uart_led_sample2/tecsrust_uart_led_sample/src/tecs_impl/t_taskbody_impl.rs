use crate::tecs_global::*;
use crate::tecs_celltype::t_taskbody::*;
use crate::tecs_signature::{s_task_body::*, s_x_uart_measure::*, s_dataqueue_rs::*};
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

					if((data == b'\n'.into()) && (lg.var.buffer_count != 0)){ // CRは13で\r、LFは10で\n
						let mut data_led: itron::dataqueue::DataElement = 0;
		
						for i in 0..lg.var.buffer_count {
							data_led *= 10;
							data_led += lg.var.buffer[i as usize] as itron::dataqueue::DataElement;
						};

						lg.var.buffer_count = 0;
		
						if let Err(_) = lg.c_dataqueue_led.send_force(&data_led) {
							lg.c_x_uart.put_char(&b'E');
						}
					}
		
					if(data >= b'0'.into() && data <= b'9'.into()){
						lg.var.buffer[lg.var.buffer_count as usize] = data as u32 - b'0' as u32;
						if((lg.var.buffer_count as usize) < (lg.var.buffer.len() - 1)){
							lg.var.buffer_count += 1;
						}
					}

					// if((data == b'\b'.into()) && (lg.var.buffer_count >= 1)){
					// 	lg.var.buffer_count -= 1;
					// }

				}
				Err(e) => {
					lg.c_x_uart.put_char(&b'E');
				}
			}
		}
	}
}

