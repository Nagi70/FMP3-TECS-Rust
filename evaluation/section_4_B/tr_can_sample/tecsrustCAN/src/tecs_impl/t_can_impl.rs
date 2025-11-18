use crate::tecs_global::*;
use crate::tecs_celltype::t_can::*;
use crate::tecs_signature::s_can_measure_for_opt::*;

use crate::x_can::*;

impl SCanMeasureForOpt for ECanForTCan{

	fn loopback_setup(&self) {
		let lg = self.cell.get_cell_ref();

		// configuration mode
        x_can_write_reg(*lg.base_address as *mut u32, XCAN_SRR_OFFSET, 0);

		while x_can_read_reg(*lg.base_address as *const u32, XCAN_SR_OFFSET) & XCAN_SR_CONFIG_MASK == 0 {}

		// set_baud_rate_prescaler
		x_can_write_reg(*lg.base_address as *mut u32, XCAN_BRPR_OFFSET, *lg.brpr_baud_prescalar as u32);

		// set_bit_timing
		let btr_value = (*lg.btr_first_timesegment as u32) & XCAN_BTR_TS1_MASK
			| ((*lg.btr_second_timesegment as u32) << XCAN_BTR_TS2_SHIFT) & XCAN_BTR_TS2_MASK
			| ((*lg.btr_sync_jump_width as u32) << XCAN_BTR_SJW_SHIFT) & XCAN_BTR_SJW_MASK;

		x_can_write_reg(*lg.base_address as *mut u32, XCAN_BTR_OFFSET, btr_value);

		// interrupt enable
		x_can_write_reg(*lg.base_address as *mut u32, XCAN_IER_OFFSET, XCAN_IXR_RXNEMP_MASK);

		// loopback mode
		x_can_write_reg(*lg.base_address as *mut u32, XCAN_MSR_OFFSET, XCAN_MSR_LBACK_MASK);
		x_can_write_reg(*lg.base_address as *mut u32, XCAN_SRR_OFFSET, XCAN_SRR_CEN_MASK);

		while x_can_read_reg(*lg.base_address as *const u32, XCAN_SR_OFFSET) & XCAN_SR_LBACK_MASK == 0 {}
	}

	fn send(&self, tx_frame: &[u32]) -> Result<(),()>{
		let lg = self.cell.get_cell_ref();

		// is_tx_fifo_full
		if (x_can_read_reg(*lg.base_address as *const u32, XCAN_SR_OFFSET) & XCAN_SR_TXFLL_MASK) != 0 {
			return Err(());
		}

        x_can_write_reg(*lg.base_address as *mut u32, XCAN_TXFIFO_ID_OFFSET, tx_frame[0]);
        x_can_write_reg(*lg.base_address as *mut u32, XCAN_TXFIFO_DLC_OFFSET, tx_frame[1]);
        x_can_write_reg(*lg.base_address as *mut u32, XCAN_TXFIFO_DW1_OFFSET, tx_frame[2]);
        x_can_write_reg(*lg.base_address as *mut u32, XCAN_TXFIFO_DW2_OFFSET, tx_frame[3]);

		Ok(())
	}
	fn receive(&self, rx_frame: &mut [u32]) -> Result<(),()> {
		let lg = self.cell.get_cell_ref();

		// is_rx_empty
		if (x_can_read_reg(*lg.base_address as *const u32, XCAN_ISR_OFFSET) & XCAN_IXR_RXNEMP_MASK) == 0 {
			return Err(());
		}

        rx_frame[0] = x_can_read_reg(*lg.base_address as *const u32, XCAN_RXFIFO_ID_OFFSET);
        rx_frame[1] = x_can_read_reg(*lg.base_address as *const u32, XCAN_RXFIFO_DLC_OFFSET);
        rx_frame[2] = x_can_read_reg(*lg.base_address as *const u32, XCAN_RXFIFO_DW1_OFFSET);
        rx_frame[3] = x_can_read_reg(*lg.base_address as *const u32, XCAN_RXFIFO_DW2_OFFSET);

		// innterrupt clear
		let intr_value = x_can_read_reg(*lg.base_address as *const u32, XCAN_ISR_OFFSET) & XCAN_IXR_RXNEMP_MASK;
        x_can_write_reg(*lg.base_address as *mut u32, XCAN_ICR_OFFSET, intr_value);

		Ok(())
	}
}

