pub trait SiSioCbr {
	fn ready_send(&'static self);
	fn ready_receive(&'static self);
}
