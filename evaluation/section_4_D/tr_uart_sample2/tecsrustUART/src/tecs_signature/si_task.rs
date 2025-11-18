use crate::tecs_global::*;
use itron::abi::*;
pub trait SiTask {
	fn activate(&self)-> ER;
	fn wakeup(&self)-> ER;
	fn release_wait(&self)-> ER;
}
