use crate::tecs_global::*;
pub trait SiTaskRs {
	fn activate(&self)-> itron::abi::ER;
	fn wakeup(&self)-> itron::abi::ER;
	fn release_wait(&self)-> itron::abi::ER;
}
