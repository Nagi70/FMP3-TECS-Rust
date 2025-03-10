use crate::{t_loop_taskbody::*, s_task_body::*, s_task_rs::*};
use crate::print;
use crate::tecs_print::*;
use itron::time::{duration, Duration, timeout, Timeout};
use itron::abi::*;
use itron::task::*;

impl STaskBody for ETaskbodyForTLoopTaskbody<'_>{

	fn main(&'static self) {
		let c_task = self.cell.get_cell_ref();
		
		loop{
			delay(duration!(s: 5)).expect("delay failed");
			let temp = c_task.get_task_state().unwrap();
			match temp {
				itron::task::State::Running => {
					print!("CAN Task: Running", );
				},
				itron::task::State::Ready => {
					print!("CAN Task: Ready", );
				},
				itron::task::State::Waiting => {
					print!("CAN Task: Waiting", );
				},
				itron::task::State::Suspended => {
					print!("CAN Task: Suspended", );
				},
				itron::task::State::WaitingSuspended => {
					print!("CAN Task: WaitingSuspended", );
				},
				itron::task::State::Dormant => {
					print!("CAN Task: Dormant", );
				},
			}
		}
	}
}

