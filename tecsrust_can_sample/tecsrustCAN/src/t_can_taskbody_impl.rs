use crate::{t_can_taskbody::*, s_can_measure::*, s_task_body::*};
use crate::print;
use crate::tecs_print::*;
use itron::time::{duration, Duration, timeout, Timeout};
use crate::x_can::*;
use itron::abi::*;
use itron::task::*;

extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

impl STaskBody for ETaskbodyForTCanTaskbody<'_>{

	fn main(&'static self) {
		let c_can = self.cell.get_cell_ref();

		print!("CAN task start",);

		delay(duration!(s: 1)).expect("delay failed");

		c_can.loopback_setup();
		print!("loopback_setup",);

		let test_message_id = 1024;
		let frame_data_length = 8;
		let mut tx_frame: [u32; 16] = [0; 16];
		let mut rx_frame: [u32; 16] = [0; 16];

		tx_frame[0] = x_can_create_id_value(test_message_id, 0, 0, 0, 0);
		print!("x_can_create_id_value",);
		tx_frame[1] = x_can_create_dlc_value(frame_data_length);
		print!("x_can_create_dlc_value",);

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

        while c_can.is_tx_fifo_full() {}

        let tx_result = c_can.send(&tx_frame);
		if !tx_result {
			print!("failure: send", );
			loop{}
		}

		while c_can.is_rx_empty() {}
		
		let rx_result = c_can.receive(&mut rx_frame);

		if rx_result {
			if rx_frame[0] != x_can_create_id_value(test_message_id, 0, 0, 0, 0) {
				print!("Loop bavck error: Invalide ID",);
			}
	
			if rx_frame[1] != x_can_create_dlc_value(frame_data_length) {
				print!("Loop bavck error: Invalide DLC",);
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
			print!("failure: receive",);
			loop{}
		}

		print!("CAN task finished",);
	}
}

