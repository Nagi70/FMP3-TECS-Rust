use crate::{t_measure::*, s_task_rs::*, s_task_body::*, s_semaphore_rs::*};
use crate::print;
use crate::tecs_print::*;
use core::num::NonZeroI32;

use itron::abi::*;
use itron::task::*;
use itron::task::State::*;
use itron::semaphore::*;
use itron::error::Error;
use itron::processor::Processor;

use itron::time::{duration, Duration, timeout, Timeout};

extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

const N: u32 = 1001;

pub static mut OVERHEAD: HrtCnt = 0;
pub static mut START: HrtCnt = 0;
pub static mut END: HrtCnt = 0;

impl STaskBody for ETaskbodyForTMeasure<'_>{

	fn main(&'static self) {
		let (c_task, c_taskmig, c_semaphore) = self.cell.get_cell_ref();

		print!("Processor1: TECS/Rust act_tsk dispatch", );
		{
			let dly_result = delay(duration!(ms: 1000));

			// match dly_result {
			// 	Ok(_) => {
			// 		print!("delay success",);
			// 	},
			// 	Err(error) => {
			// 		match error {
			// 			BadContext => {
			// 				print!("BadContext", );
			// 			},
			// 			NotSupported => {
			// 				print!("BadId", );
			// 			},
			// 			Released => {
			// 				print!("BadState", );
			// 			},
			// 			TerminateRequest => {
			// 				print!("AccessDenied", );
			// 			},
			// 		}
			// 	},
			// }
		}

		let mut dispatch_time :HrtCnt = 0;
		let mut dispatch_end :HrtCnt = 0;
		let mut overhead :HrtCnt = 0;

		unsafe{ dispatch_time = fch_hrt();}
		for i in 0..N {
			unsafe{ dispatch_end = fch_hrt();}
		}

		overhead = (dispatch_end - dispatch_time) / N;

		unsafe{
			OVERHEAD = overhead;
		}

		let mut start :HrtCnt = 0;
		let mut end :HrtCnt = 0;
		let mut duration :HrtCnt = 0;

		let mut act_result :Result<(), Error<ActivateError>> = Ok(());
		let mut acto_result :Result<(), Error<ActivateOnError>> = Ok(());
		let mut get_result :Result<Priority, Error<PriorityError>> = Ok(0);
		let mut chg_result :Result<(), Error<SetPriorityError>> = Ok(());
		let mut mig_result :Result<(), Error<MigrateError>> = Ok(());
		let mut ter_result :Result<(), Error<TerminateError>> = Ok(());

		let mut sig_result :Result<(), Error<SignalError>> = Ok(());
		let mut wait_result :Result<(), Error<WaitError>> = Ok(());

		let set_priority :Priority = 5;
		let default_priority :Priority = 10;

		let processor1 = Processor::from_raw_nonnull(NonZeroI32::new(1).unwrap());
		let processor2 = Processor::from_raw_nonnull(NonZeroI32::new(2).unwrap());

		let mut can_result :Result<usize, Error<CancelActivateAllError>> = Ok(0);

		{
			let dly_result = delay(duration!(ms: 1000));
		}

		for i in 0..N{

        	/* c_task.act_tsk(); ↓ TASK2_2(attr: TA_NULL, pri: 5) */
			// {
			// 	while c_task.refer().unwrap().state() != Dormant {
			// 		delay(duration!(ms: 5)).expect("delay failed");
			// 	}
			// }


			/* c_task.change_priority(&set_priority); ↓ TASK2_2(attr: TA_ACT, pri: 10) */
			{
				while (c_task.refer().unwrap().state() == Running || c_task.get_priority().unwrap() != default_priority){
					delay(duration!(ms: 5)).expect("delay failed");
				}
				c_task.change_priority(&default_priority);
				c_task.activate();
			}


			/* c_taskmig.migrate(&processor2); ↓ TASK_MIG(attr: TA_NULL, pri: 6) */
			// {
			// 	while c_taskmig.refer().unwrap().state() != Dormant {
			// 		delay(duration!(ms: 5)).expect("delay failed");
			// 	}
			// 	c_taskmig.activate();
			// }


			// {
			// 	let refer = c_task.refer();
			// 	match refer {
			// 		Ok(info) => {
			// 			match info.state() {
			// 				Running => {
			// 					print!("Running", );
			// 				},
			// 				Ready => {
			// 					print!("Ready", );
			// 				},
			// 				Waiting => {
			// 					print!("Waiting", );
			// 				},
			// 				Suspended => {
			// 					print!("Suspended", );
			// 				},
			// 				WaitingSuspended => {
			// 					print!("WaitingSuspended", );
			// 				},
			// 				Dormant => {
			// 					print!("Dormant", );
			// 				},
			// 			}
			// 		},
			// 		Err(_) => {
			// 			print!("info error", );
			// 		},
			// 	}
			// }

			// {
			// 	let dly_result = delay(duration!(ms: 20));
			// }

			unsafe{ 
				// _ = loc_cpu();
				START = fch_hrt();
			}

			// act_result = c_task.activate();
			chg_result = c_task.change_priority(&set_priority);
			// mig_result = c_taskmig.migrate(&processor2); // mig_tsk は 呼び出したタスクと同じプロセッサに割り付けられているタスクのみに適用可能
			


			// let refer = c_task.refer();
			// match refer {
			// 	Ok(info) => {
			// 		match info.state() {
			// 			Running => {
			// 				print!("Running", );
			// 			},
			// 			Ready => {
			// 				print!("Ready", );
			// 			},
			// 			Waiting => {
			// 				print!("Waiting", );
			// 			},
			// 			Suspended => {
			// 				print!("Suspended", );
			// 			},
			// 			WaitingSuspended => {
			// 				print!("WaitingSuspended", );
			// 			},
			// 			Dormant => {
			// 				print!("Dormant", );
			// 			},
			// 		}
			// 	},
			// 	Err(_) => {
			// 		print!("info error", );
			// 	},
			// }

			// c_task.activate() ↓
			// sig_result = c_semaphore.signal();
			// match act_result {
			// 	Ok(_) => {
			// 		print!("activation success",);
			// 	},
			// 	Err(error) => {
			// 		match error {
			// 			BadContext => {
			// 				print!("BadContext", );
			// 			},
			// 			BadId => {
			// 				print!("BadId", );
			// 			},
			// 			BadState => {
			// 				print!("BadState", );
			// 			},
			// 			AccessDenied => {
			// 				print!("AccessDenied", );
			// 			},
			// 		}
			// 	},
			// }

			// c_task.get_priority(); ↓
			// sig_result = c_semaphore.signal();
			// match get_result {
			// 	Ok(pri) => {
			// 		print!("get_pri succcess %tu", pri);
			// 	},
			// 	Err(_) => {
			// 		print!("get_pri error", );
			// 	},
			// }

			// c_task.migrate_and_activate(&processor2) ↓
			// c_task.terminate();
			// c_task.migrate(&processor1);

			// c_task.change_priority(&set_priority) ↓
			// sig_result = c_semaphore.signal();
			// get_result = c_task.get_priority();
			// match get_result {
			// 	Ok(pri) => {
			// 		print!("get_pri succcess %tu", pri);
			// 	},
			// 	Err(_) => {
			// 		print!("get_pri error", );
			// 	},
			// }
			// match chg_result {
			// 	Ok(_) => {
			// 		print!("chg_pri succcess", );
			// 	},
			// 	Err(_) => {
			// 		print!("chg_pri error", );
			// 	},
			// }
			// chg_result = c_task.change_priority(&default_priority);
			// get_result = c_task.get_priority();
			// match get_result {
			// 	Ok(pri) => {
			// 		print!("get_pri succcess %tu", pri);
			// 	},
			// 	Err(_) => {
			// 		print!("get_pri error", );
			// 	},
			// }

			// c_taskmig.migrate(&processor2) ↓
			// sig_result = c_semaphore.signal();
			// match mig_result {
			// 	Ok(_) => {
			// 		print!("mig_tsk succcess", );
			// 	},
			// 	Err(error) => {
			// 		match error {
			// 			BadContext => {
			// 				print!("BadContext", );
			// 			},
			// 			BadId => {
			// 				print!("BadId", );
			// 			},
			// 			AccessDenied => {
			// 				print!("AccessDenied", );
			// 			},
			// 			BadParam => {
			// 				print!("BadParam", );
			// 			},
			// 		}
			// 	},
			// }
			
			// c_taskmig.terminate() ↓
			// sig_result = c_semaphore.signal();
			// match ter_result {
			// 	Ok(_) => {
			// 		print!("ter_tsk succcess", );
			// 	},
			// 	Err(error) => {
			// 		match error {
			// 			BadContext => {
			// 				print!("BadContext", );
			// 			},
			// 			BadId => {
			// 				print!("BadId", );
			// 			},
			// 			AccessDenied => {
			// 				print!("AccessDenied", );
			// 			},
			// 			BadState => {
			// 				print!("BadState", );
			// 			},
			// 			BadParam => {
			// 				print!("BadParam", );
			// 			},
			// 		}
			// 	},
			// }
			// let refer2 = c_taskmig.refer();
			// match refer2 {
			// 	Ok(info) => {
			// 		match info.state() {
			// 			Running => {
			// 				print!("Running", );
			// 			},
			// 			Ready => {
			// 				print!("Ready", );
			// 			},
			// 			Waiting => {
			// 				print!("Waiting", );
			// 			},
			// 			Suspended => {
			// 				print!("Suspended", );
			// 			},
			// 			WaitingSuspended => {
			// 				print!("WaitingSuspended", );
			// 			},
			// 			Dormant => {
			// 				print!("Dormant", );
			// 			},
			// 		}
			// 	},
			// 	Err(_) => {
			// 		print!("info error", );
			// 	},
			// }
			// act_result = c_taskmig.activate();
			// match act_result {
			// 	Ok(_) => {
			// 		print!("act_tsk succcess", );
			// 	},
			// 	Err(_) => {
			// 		print!("act_tsk error", );
			// 	},
			// }

			// delay(duration!(ms: 50)).expect("delay failed");
		}
		
	}
}

