use crate::tecs_global::*;
pub trait SLed {
	fn set_up(&self);
	fn light_on(&self);
	fn light_off(&self);
}
