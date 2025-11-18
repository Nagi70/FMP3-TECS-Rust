pub trait SCanMeasure {
	fn loopback_setup(&'static self);
	fn send(&'static self, tx_frame: &[u32; 16])-> bool;
	fn receive(&'static self, rx_frame: &mut [u32; 16])-> bool;
	fn status(&'static self)-> u32;
	fn mode(&'static self)-> u8;
	fn enter_mode(&'static self, operation_mode: &u8)-> bool;
	fn set_baud_rate_prescaler(&'static self)-> bool;
	fn set_bit_timing(&'static self)-> bool;
	fn is_tx_fifo_full(&'static self)-> bool;
	fn is_rx_empty(&'static self)-> bool;
	fn interrupt_enable(&'static self, mask: &u32);
	fn interrupt_clear(&'static self, mask: &u32);
	fn interrupt_status(&'static self)-> u32;
}
