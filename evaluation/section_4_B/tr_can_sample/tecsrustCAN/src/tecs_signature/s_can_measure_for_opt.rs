use crate::tecs_global::*;
pub trait SCanMeasureForOpt {
	fn loopback_setup(&self);
	fn send(&self, tx_frame: &[u32])-> Result<(),()>;
	fn receive(&self, rx_frame: &mut [u32])-> Result<(),()>;
}
