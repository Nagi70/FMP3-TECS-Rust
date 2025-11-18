use crate::tecs_global::*;
pub trait SiSemaphore {
	fn signal(&self)-> ER;
}
