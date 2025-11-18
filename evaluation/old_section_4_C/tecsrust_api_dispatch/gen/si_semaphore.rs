pub trait SiSemaphore {
	fn signal(&'static self)-> ER;
}
