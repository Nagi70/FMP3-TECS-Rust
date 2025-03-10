use crate::{t_can::*, s_can_measure::*};
use crate::x_can::*;
use crate::print;
use crate::tecs_print::*;
use itron::task::*;
use itron::time::{duration, Duration, timeout, Timeout};

impl SCanMeasure for ECanForTCan<'_>{

	fn loopback_setup(&'static self) {
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
		print!("loopback_setup start",);

        delay(duration!(ms: 100)).expect("delay failed");

        if !self.enter_mode(&XCAN_MODE_CONFIG) {
            print!("failure: enter_mode(&XCAN_MODE_CONFIG)",);
            delay(duration!(ms: 100)).expect("delay failed");
        }

        while self.mode() != XCAN_MODE_CONFIG {}

        self.set_baud_rate_prescaler();
        self.set_bit_timing();

        self.interrupt_enable(&XCAN_IXR_RXNEMP_MASK);

        if !self.enter_mode(&XCAN_MODE_LOOPBACK){
            print!("failure: enter_mode(&XCAN_MODE_LOOPBACK)",);
            delay(duration!(ms: 100)).expect("delay failed");
        }
        while self.mode() != XCAN_MODE_LOOPBACK {}

		print!("loopback_setup finished",);
        delay(duration!(ms: 100)).expect("delay failed");
	}

	fn send(&'static self, tx_frame: &[u32; 16]) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

        if self.is_tx_fifo_full() {
            return false;
        }

        unsafe{
            x_can_write_reg(*base_address as *mut u32, XCAN_TXFIFO_ID_OFFSET, tx_frame[0]);
            x_can_write_reg(*base_address as *mut u32, XCAN_TXFIFO_DLC_OFFSET, tx_frame[1]);
            x_can_write_reg(*base_address as *mut u32, XCAN_TXFIFO_DW1_OFFSET, tx_frame[2]);
            x_can_write_reg(*base_address as *mut u32, XCAN_TXFIFO_DW2_OFFSET, tx_frame[3]);
        }

        true
	}

	fn receive(&'static self, rx_frame: &mut [u32; 16]) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

        if self.is_rx_empty() {
            return false;
        }

        unsafe{
            rx_frame[0] = x_can_read_reg(*base_address as *const u32, XCAN_RXFIFO_ID_OFFSET);
            rx_frame[1] = x_can_read_reg(*base_address as *const u32, XCAN_RXFIFO_DLC_OFFSET);
            rx_frame[2] = x_can_read_reg(*base_address as *const u32, XCAN_RXFIFO_DW1_OFFSET);
            rx_frame[3] = x_can_read_reg(*base_address as *const u32, XCAN_RXFIFO_DW2_OFFSET);
        }

        self.interrupt_clear(&XCAN_IXR_RXNEMP_MASK);

        true
	}

	fn status(&'static self) -> u32{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
		print!("status start",);
        delay(duration!(ms: 100)).expect("delay failed");

        unsafe{
		    print!("status finished",);
            delay(duration!(ms: 100)).expect("delay failed");

            print!("base_address: %tX", *base_address as u32);
            delay(duration!(ms: 100)).expect("delay failed");

		    x_can_read_reg(*base_address as *const u32, XCAN_SR_OFFSET)
		    // let temp = x_can_read_reg(*base_address as *const u32, XCAN_SR_OFFSET);
            // print!("temp return",);
            // delay(duration!(ms: 100)).expect("delay failed");
            // temp
        }
	}
	fn mode(&'static self) -> u8{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
		print!("mode start",);
        delay(duration!(ms: 100)).expect("delay failed");

		let value = self.status();

		print!("mode finished",);
        delay(duration!(ms: 100)).expect("delay failed");

        if value & XCAN_SR_CONFIG_MASK != 0 {
            XCAN_MODE_CONFIG
        } else if value & XCAN_SR_SLEEP_MASK != 0 {
            XCAN_MODE_SLEEP
        } else if value & XCAN_SR_NORMAL_MASK != 0 {
            XCAN_MODE_NORMAL
        } else {
            XCAN_MODE_LOOPBACK
        }
	}

	fn enter_mode(&'static self, operation_mode: &u8)-> bool {
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
		print!("enter_mode start",);
        delay(duration!(ms: 100)).expect("delay failed");

        let current_mode = self.mode();

        if (current_mode == XCAN_MODE_NORMAL && *operation_mode == XCAN_MODE_SLEEP) ||
           (current_mode == XCAN_MODE_SLEEP && *operation_mode == XCAN_MODE_NORMAL) {
            
            let value = if *operation_mode == XCAN_MODE_SLEEP {
                XCAN_MSR_SLEEP_MASK
            } else {
                0
            };
            
            unsafe{
		        print!("enter_mode: write_reg(XCAN_MSR_OFFSET)",);
                delay(duration!(ms: 100)).expect("delay failed");
                x_can_write_reg(*base_address as *mut u32, XCAN_MSR_OFFSET, value);
            }
		    print!("enter_mode finished",);
            delay(duration!(ms: 100)).expect("delay failed");
            return true;
        }

		print!("enter_mode: write_reg(XCAN_SRR_OFFSET)",);
        delay(duration!(ms: 100)).expect("delay failed");
        // Configuration Mode に入る
        unsafe{
            x_can_write_reg(*base_address as *mut u32, XCAN_SRR_OFFSET, 0);
        }

        if self.mode() != XCAN_MODE_CONFIG {
            return false;
        }

        print!("enter_mode: write_reg(XCAN_MSR_OFFSET) and write_reg(XCAN_SRR_OFFSET)",);
        delay(duration!(ms: 100)).expect("delay failed");
        match *operation_mode {
            XCAN_MODE_CONFIG => {}, // 既に Configuration Mode のため何もしない
            XCAN_MODE_SLEEP => {
                unsafe{
                    x_can_write_reg(*base_address as *mut u32, XCAN_MSR_OFFSET, XCAN_MSR_SLEEP_MASK);
                    x_can_write_reg(*base_address as *mut u32, XCAN_SRR_OFFSET, XCAN_SRR_CEN_MASK);
                }
            }
            XCAN_MODE_NORMAL => {
                unsafe{
                    x_can_write_reg(*base_address as *mut u32, XCAN_MSR_OFFSET, 0);
                    x_can_write_reg(*base_address as *mut u32, XCAN_SRR_OFFSET, XCAN_SRR_CEN_MASK);
                }
            }
            XCAN_MODE_LOOPBACK => {
                unsafe{
                    x_can_write_reg(*base_address as *mut u32, XCAN_MSR_OFFSET, XCAN_MSR_LBACK_MASK);
                    x_can_write_reg(*base_address as *mut u32, XCAN_SRR_OFFSET, XCAN_SRR_CEN_MASK);
                }
            }
            _ => unreachable!(),
        }

		print!("enter_mode finished",);
        delay(duration!(ms: 100)).expect("delay failed");

        true
    }
	fn set_baud_rate_prescaler(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
		print!("set_baud_rate_prescaler start",);

		if self.mode() != XCAN_MODE_CONFIG {
			return false;
		}

        unsafe{
		    x_can_write_reg(*base_address as *mut u32, XCAN_BRPR_OFFSET, *brpr_baud_prescalar as u32);
        }

		print!("set_baud_rate_prescaler finished",);

		true
	}
	fn set_bit_timing(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
		print!("set_bit_timing start",);

		if *btr_sync_junp_width > 3 || *btr_second_timesegment > 7 || *btr_first_timesegment > 15 {
			return false;
		}

		if self.mode() != XCAN_MODE_CONFIG {
			return false;
		}

		let mut value = (*btr_first_timesegment as u32) & XCAN_BTR_TS1_MASK;
		value |= ((*btr_second_timesegment as u32) << XCAN_BTR_TS2_SHIFT) & XCAN_BTR_TS2_MASK;
		value |= ((*btr_sync_junp_width as u32) << XCAN_BTR_SJW_SHIFT) & XCAN_BTR_SJW_MASK;

        unsafe{
		    x_can_write_reg(*base_address as *mut u32, XCAN_BTR_OFFSET, value);
        }

		print!("set_bit_timing finished",);

		true
	}

    fn is_tx_fifo_full(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
        
        unsafe{
            (x_can_read_reg(*base_address as *const u32, XCAN_SR_OFFSET) & XCAN_SR_TXFLL_MASK) != 0
        }
    }

    fn is_rx_empty(&'static self) -> bool{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
        
        unsafe{
            (x_can_read_reg(*base_address as *const u32, XCAN_ISR_OFFSET) & XCAN_IXR_RXNEMP_MASK) == 0
        }
    }

	fn interrupt_enable(&'static self, mask: &u32){
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();
        
        unsafe{
		    x_can_write_reg(*base_address as *mut u32, XCAN_IER_OFFSET, *mask);
        }
    }

    fn interrupt_clear(&'static self, mask: &u32){
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

        let mut intr_value = self.interrupt_status();
        intr_value &= *mask;
        unsafe{
            x_can_write_reg(*base_address as *mut u32, XCAN_ICR_OFFSET, intr_value);
        }
    }

	fn interrupt_status(&'static self) -> u32{
		let (base_address, message_id, frame_data_length, brpr_baud_prescalar, btr_sync_junp_width, btr_second_timesegment, btr_first_timesegment) = self.cell.get_cell_ref();

        unsafe{
            x_can_read_reg(*base_address as *const u32, XCAN_ISR_OFFSET)
        }
    }

}

