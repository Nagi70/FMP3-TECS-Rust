use crate::{t_led_taskbody::*, s_led::*, s_task_body::*};

use crate::print;
use crate::tecs_print::*;
use itron::task::*;
use itron::time::{duration, Duration, timeout, Timeout};

use crate::kernel_cfg::*;
use itron::semaphore::SemaphoreRef;
use core::num::NonZeroI32;

const semaphore: SemaphoreRef = unsafe{SemaphoreRef::from_raw_nonnull(NonZeroI32::new(SEMID_1).unwrap())};

impl STaskBody for ETaskbodyForTLedTaskbody<'_>{

	fn main(&'static self) {
		let c_led = self.cell.get_cell_ref();
		print!("Processor1: LED task start", );

		print!("LED setup", );
		c_led.set_up();
		delay(duration!(ms: 1000)).expect("delay failed");

		loop{
			semaphore.wait();
			print!("LED ON", );
			semaphore.signal();
			c_led.light_on();
			delay(duration!(ms: 1000)).expect("delay failed");

			semaphore.wait();
			print!("LED OFF", );
			semaphore.signal();
			c_led.light_off();
			delay(duration!(ms: 1000)).expect("delay failed");
			
		}
	}
}

