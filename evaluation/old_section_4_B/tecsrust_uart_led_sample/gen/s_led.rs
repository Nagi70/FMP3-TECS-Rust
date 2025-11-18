pub trait SLed {
	fn set_up(&'static self);
	fn light_on(&'static self);
	fn light_off(&'static self);
}
