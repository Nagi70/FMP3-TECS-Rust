use crate::tecs_global::*;
pub trait SiSioCbr {
	fn ready_send(&self);
	fn ready_receive(&self);
}
