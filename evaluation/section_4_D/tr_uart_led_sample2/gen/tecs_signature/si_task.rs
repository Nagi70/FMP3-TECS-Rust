use crate::tecs_global::*;
pub trait SiTask {
	fn activate(&self)-> ER;
	fn wakeup(&self)-> ER;
	fn release_wait(&self)-> ER;
}
