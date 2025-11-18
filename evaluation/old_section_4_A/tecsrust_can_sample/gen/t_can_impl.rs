use crate::{t_can::*, s_can_measure::*};

impl SCanMeasure for ECanForTCan<'_>{

	fn loopback_setup(&'static self) {
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn send(&'static self, tx_frame: &[u32; 16]) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn receive(&'static self, rx_frame: &mut [u32; 16]) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn status(&'static self) -> u32{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn mode(&'static self) -> u8{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn enter_mode(&'static self, operation_mode: &u8) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn set_baud_rate_prescaler(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn set_bit_timing(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn is_tx_fifo_full(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn is_rx_empty(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn interrupt_enable(&'static self, mask: &u32) {
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn interrupt_clear(&'static self, mask: &u32) {
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
	fn interrupt_status(&'static self) -> u32{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

	}
}

