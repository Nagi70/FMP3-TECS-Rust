use crate::tecs_global::*;
use crate::tecs_celltype::t_measure::*;
use crate::tecs_signature::{s_task_rs::*, s_task_body::*};

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

unsafe extern "C" {
	fn fch_hrt() -> HrtCnt;
	fn loc_cpu() -> ER;
	fn unl_cpu() -> ER;
	fn dis_dsp() -> ER;
	fn ena_dsp() -> ER;
}

const N: u32 = 1000;

pub static mut OVERHEAD: HrtCnt = 0;
pub static mut START: HrtCnt = 0;
pub static mut END: HrtCnt = 0;

pub static mut START_U: u32 = 0;
pub static mut START_L: u32 = 0;
pub static mut END_U: u32 = 0;
pub static mut END_L: u32 = 0;

impl STaskBody for ETaskbodyForTMeasure{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

		#[cfg(feature = "act_tsk")]
		{
			print!("Evaluate act_tsk with dispatch", );
		}

		#[cfg(feature = "chg_pri")]
		{
			print!("Evaluate chg_pri with dispatch", );
		}

		#[cfg(feature = "mig_tsk")]
		{
			print!("Evaluate mig_tsk with dispatch", );
		}

		delay(duration!(ms: 1000)).expect("delay failed");

		let mut dispatch_time :HrtCnt = 0;
		let mut dispatch_end :HrtCnt = 0;
		let mut overhead :HrtCnt = 0;

		unsafe{
			START_U = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
			START_L = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
		}
		for i in 0..N {
			unsafe{
				END_U = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				END_L = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}
		}

		unsafe {
			let cnt64_overhead_start = ((START_U as u64) << 32) | (START_L as u64);
			dispatch_time = cnt64_overhead_start as crate::tecs_print::HrtCnt;

			let cnt64_overhead_end = ((END_U as u64) << 32) | (END_L as u64);
			dispatch_end = cnt64_overhead_end as crate::tecs_print::HrtCnt;
		}

		overhead = (dispatch_end - dispatch_time) / N;

		unsafe{
			OVERHEAD = overhead;
		}

		print!("Overhead: %tu", overhead);

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

		delay(duration!(ms: 1000)).expect("delay failed");

		for i in 0..N{
			#[cfg(feature = "act_tsk")] /* TASK2_2(attr: TA_NULL, pri: 5) */
			{
				while lg.c_task.refer().unwrap().state() != Dormant {
					delay(duration!(ms: 5)).expect("delay failed");
				}
			}

			#[cfg(feature = "chg_pri")] /* TASK2_2(attr: TA_ACT, pri: 10) */
			{
				// delay(duration!(ms: 50)).expect("delay failed");
				while (lg.c_task.refer().unwrap().state() == Running || lg.c_task.get_priority().unwrap() != default_priority){
					delay(duration!(ms: 5)).expect("delay failed");
				}
				lg.c_task.change_priority(&default_priority);
				lg.c_task.activate();
			}

			#[cfg(feature = "mig_tsk")] /* TASK_MIG(attr: TA_NULL, pri: 6) */
			{
				while lg.c_taskmig.refer().unwrap().state() != Dormant {
					delay(duration!(ms: 5)).expect("delay failed");
				}
				lg.c_taskmig.activate();
			}

			#[cfg(all(feature = "debug_task_info", any(feature = "act_tsk", feature = "chg_pri")))]
			{
				let refer = lg.c_task.refer();
				match refer {
					Ok(info) => {
						match info.state() {
							Running => {
								print!("Before Status: Running", );
							},
							Ready => {
								print!("Before Status: Ready", );
							},
							Waiting => {
								print!("Before Status: Waiting", );
							},
							Suspended => {
								print!("Before Status: Suspended", );
							},
							WaitingSuspended => {
								print!("Before Status: WaitingSuspended", );
							},
							Dormant => {
								print!("Before Status: Dormant", );
							},
						}
					},
					Err(_) => {
						print!("info error", );
					},
				}

				let dly_result = delay(duration!(ms: 20));
			}

			#[cfg(all(feature = "debug_task_info", feature = "mig_tsk"))]
			{
				let refer = lg.c_taskmig.refer();
				match refer {
					Ok(info) => {
						match info.state() {
							Running => {
								print!("Before Status: Running", );
							},
							Ready => {
								print!("Before Status: Ready", );
							},
							Waiting => {
								print!("Before Status: Waiting", );
							},
							Suspended => {
								print!("Before Status: Suspended", );
							},
							WaitingSuspended => {
								print!("Before Status: WaitingSuspended", );
							},
							Dormant => {
								print!("Before Status: Dormant", );
							},
						}
					},
					Err(_) => {
						print!("info error", );
					},
				}

				let dly_result = delay(duration!(ms: 20));
			}

			unsafe{ 
				START_U = core::ptr::read_volatile(0xF8F00204 as *const u32); // COUNT_U
				START_L = core::ptr::read_volatile(0xF8F00200 as *const u32); // COUNT_L
			}

			#[cfg(feature = "act_tsk")]
			{	act_result = lg.c_task.activate();	}
			#[cfg(feature = "chg_pri")]
			{	chg_result = lg.c_task.change_priority(&set_priority);	}
			#[cfg(feature = "mig_tsk")]
			{	mig_result = lg.c_taskmig.migrate(&processor2);	}

			#[cfg(all(feature = "debug_task_info", any(feature = "act_tsk", feature = "chg_pri")))]
			{
				let refer = lg.c_task.refer();
				match refer {
					Ok(info) => {
						match info.state() {
							Running => {
								print!("After Status: Running", );
							},
							Ready => {
								print!("After Status: Ready", );
							},
							Waiting => {
								print!("After Status: Waiting", );
							},
							Suspended => {
								print!("After Status: Suspended", );
							},
							WaitingSuspended => {
								print!("After Status: WaitingSuspended", );
							},
							Dormant => {
								print!("After Status: Dormant", );
							},
						}
					},
					Err(_) => {
						print!("info error", );
					},
				}

				let dly_result = delay(duration!(ms: 20));
			}

			#[cfg(all(feature = "debug_task_info", feature = "mig_tsk"))]
			{
				let refer = lg.c_taskmig.refer();
				match refer {
					Ok(info) => {
						match info.state() {
							Running => {
								print!("Before Status: Running", );
							},
							Ready => {
								print!("Before Status: Ready", );
							},
							Waiting => {
								print!("Before Status: Waiting", );
							},
							Suspended => {
								print!("Before Status: Suspended", );
							},
							WaitingSuspended => {
								print!("Before Status: WaitingSuspended", );
							},
							Dormant => {
								print!("Before Status: Dormant", );
							},
						}
					},
					Err(_) => {
						print!("info error", );
					},
				}

				let dly_result = delay(duration!(ms: 20));
			}

			#[cfg(all(feature = "act_tsk", feature = "debug_task_info"))]
			{
				match act_result {
					Ok(_) => {
						print!("activation success",);
					},
					Err(error) => {
						match error {
							BadContext => {
								print!("BadContext", );
							},
							BadId => {
								print!("BadId", );
							},
							BadState => {
								print!("BadState", );
							},
							AccessDenied => {
								print!("AccessDenied", );
							},
						}
					},
				}
			}

			#[cfg(all(feature = "chg_pri", feature = "debug_task_info"))]
			{
				match chg_result {
					Ok(_) => {
						print!("chg_pri success", );
					},
					Err(_) => {
						print!("chg_pri error", );
					},
				}
			}

			#[cfg(all(feature = "mig_tsk", feature = "debug_task_info"))]
			{
				match mig_result {
					Ok(_) => {
						print!("mig_tsk success", );
					},
					Err(error) => {
						match error {
							BadContext => {
								print!("BadContext", );
							},
							BadId => {
								print!("BadId", );
							},
							AccessDenied => {
								print!("AccessDenied", );
							},
							BadParam => {
								print!("BadParam", );
							},
						}
					},
				}
			}

		}
	}
}

