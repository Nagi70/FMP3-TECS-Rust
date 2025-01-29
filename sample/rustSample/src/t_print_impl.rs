use crate::{t_print::*, s_task_body::*};
use crate::print;
use crate::print::*;
use itron::task::{sleep, sleep_timeout, exit, delay};
use itron::time::{duration, Duration, timeout, Timeout};

impl STaskBody for EPrintForTPrint<'_>{

	fn main(&self) {
		delay(duration!(s: 1)).expect("delay failed");
		loop{
			let mut prcid = 0;

			sil_get_pid(&mut prcid);

			if prcid == 1 {
				print!("Processor %d, delay 1s ...", prcid);
				delay(duration!(s: 1)).expect("delay failed");
			}
			else if prcid == 2 {
				print!("			Processor %d, delay 2s ...", prcid);
				delay(duration!(s: 2)).expect("delay failed");
			}
		}
	}
}

use core::arch::asm;

fn sil_get_pid(p_prcid: &mut i32) {
    let mut reg: u32;

    unsafe {
        asm!(
            "mrc p15, 0, {0}, c0, c0, 5",
            out(reg) reg
        );
    }

    *p_prcid = ((reg & 0x0ff) as i32) + 1;
}

