use itron::abi::*;

pub trait SiSemaphore {
	fn signal(&'static self)-> ER;
}
