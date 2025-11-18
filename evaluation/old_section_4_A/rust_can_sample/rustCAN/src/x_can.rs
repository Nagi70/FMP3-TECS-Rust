// SPDX-License-Identifier: MIT

/// CAN register offsets
pub const XCAN_SRR_OFFSET: u32 = 0x000;
pub const XCAN_MSR_OFFSET: u32 = 0x004;
pub const XCAN_BRPR_OFFSET: u32 = 0x008;
pub const XCAN_BTR_OFFSET: u32 = 0x00C;
pub const XCAN_ECR_OFFSET: u32 = 0x010;
pub const XCAN_ESR_OFFSET: u32 = 0x014;
pub const XCAN_SR_OFFSET: u32 = 0x018;
pub const XCAN_ISR_OFFSET: u32 = 0x01C;
pub const XCAN_IER_OFFSET: u32 = 0x020;
pub const XCAN_ICR_OFFSET: u32 = 0x024;
pub const XCAN_TXFIFO_ID_OFFSET: u32 = 0x030;
pub const XCAN_TXFIFO_DLC_OFFSET: u32 = 0x034;
pub const XCAN_TXFIFO_DW1_OFFSET: u32 = 0x038;
pub const XCAN_TXFIFO_DW2_OFFSET: u32 = 0x03C;
pub const XCAN_TXBUF_ID_OFFSET: u32 = 0x040;
pub const XCAN_TXBUF_DLC_OFFSET: u32 = 0x044;
pub const XCAN_TXBUF_DW1_OFFSET: u32 = 0x048;
pub const XCAN_TXBUF_DW2_OFFSET: u32 = 0x04C;
pub const XCAN_RXFIFO_ID_OFFSET: u32 = 0x050;
pub const XCAN_RXFIFO_DLC_OFFSET: u32 = 0x054;
pub const XCAN_RXFIFO_DW1_OFFSET: u32 = 0x058;
pub const XCAN_RXFIFO_DW2_OFFSET: u32 = 0x05C;
pub const XCAN_AFR_OFFSET: u32 = 0x060;
pub const XCAN_AFMR1_OFFSET: u32 = 0x064;
pub const XCAN_AFIR1_OFFSET: u32 = 0x068;
pub const XCAN_AFMR2_OFFSET: u32 = 0x06C;
pub const XCAN_AFIR2_OFFSET: u32 = 0x070;
pub const XCAN_AFMR3_OFFSET: u32 = 0x074;
pub const XCAN_AFIR3_OFFSET: u32 = 0x078;
pub const XCAN_AFMR4_OFFSET: u32 = 0x07C;
pub const XCAN_AFIR4_OFFSET: u32 = 0x080;
pub const XCAN_ECC_CFG_OFFSET: u32 = 0x0C8;
pub const XCAN_TXTLFIFO_ECC_OFFSET: u32 = 0x0CC;
pub const XCAN_TXOLFIFO_ECC_OFFSET: u32 = 0x0D0;
pub const XCAN_RXFIFO_ECC_OFFSET: u32 = 0x0D4;

/// Software Reset Register
pub const XCAN_SRR_CEN_MASK: u32 = 0x00000002;
pub const XCAN_SRR_SRST_MASK: u32 = 0x00000001;

/// Mode Select Register
pub const XCAN_MSR_LBACK_MASK: u32 = 0x00000002;
pub const XCAN_MSR_SLEEP_MASK: u32 = 0x00000001;

/// Baud Rate Prescaler register
pub const XCAN_BRPR_BRP_MASK: u32 = 0x000000FF;

/// Bit Timing Register
pub const XCAN_BTR_SJW_MASK: u32 = 0x00000180;
pub const XCAN_BTR_SJW_SHIFT: u32 = 7;
pub const XCAN_BTR_TS2_MASK: u32 = 0x00000070;
pub const XCAN_BTR_TS2_SHIFT: u32 = 4;
pub const XCAN_BTR_TS1_MASK: u32 = 0x0000000F;

/// Error Counter Register
pub const XCAN_ECR_REC_MASK: u32 = 0x0000FF00;
pub const XCAN_ECR_REC_SHIFT: u32 = 8;
pub const XCAN_ECR_TEC_MASK: u32 = 0x000000FF;

/// Error Status Register
pub const XCAN_ESR_ACKER_MASK: u32 = 0x00000010;
pub const XCAN_ESR_BERR_MASK: u32 = 0x00000008;
pub const XCAN_ESR_STER_MASK: u32 = 0x00000004;
pub const XCAN_ESR_FMER_MASK: u32 = 0x00000002;
pub const XCAN_ESR_CRCER_MASK: u32 = 0x00000001;

// SPDX-License-Identifier: MIT

/// Status Register
pub const XCAN_SR_ACFBSY_MASK: u32 = 0x00000800;
pub const XCAN_SR_TXFLL_MASK: u32 = 0x00000400;
pub const XCAN_SR_TXBFLL_MASK: u32 = 0x00000200;
pub const XCAN_SR_ESTAT_MASK: u32 = 0x00000180;
pub const XCAN_SR_ESTAT_SHIFT: u32 = 7;
pub const XCAN_SR_ERRWRN_MASK: u32 = 0x00000040;
pub const XCAN_SR_BBSY_MASK: u32 = 0x00000020;
pub const XCAN_SR_BIDLE_MASK: u32 = 0x00000010;
pub const XCAN_SR_NORMAL_MASK: u32 = 0x00000008;
pub const XCAN_SR_SLEEP_MASK: u32 = 0x00000004;
pub const XCAN_SR_LBACK_MASK: u32 = 0x00000002;
pub const XCAN_SR_CONFIG_MASK: u32 = 0x00000001;

/// Interrupt Status/Enable/Clear Register
pub const XCAN_IXR_E2BERX_MASK: u32 = 0x00800000;
pub const XCAN_IXR_E1BERX_MASK: u32 = 0x00400000;
pub const XCAN_IXR_E2BETXOL_MASK: u32 = 0x00200000;
pub const XCAN_IXR_E1BETXOL_MASK: u32 = 0x00100000;
pub const XCAN_IXR_E2BETXTL_MASK: u32 = 0x00080000;
pub const XCAN_IXR_E1BETXTL_MASK: u32 = 0x00040000;

pub const XCAN_IXR_WKUP_MASK: u32 = 0x00000800;
pub const XCAN_IXR_SLP_MASK: u32 = 0x00000400;
pub const XCAN_IXR_BSOFF_MASK: u32 = 0x00000200;
pub const XCAN_IXR_ERROR_MASK: u32 = 0x00000100;
pub const XCAN_IXR_RXNEMP_MASK: u32 = 0x00000080;
pub const XCAN_IXR_RXOFLW_MASK: u32 = 0x00000040;
pub const XCAN_IXR_RXUFLW_MASK: u32 = 0x00000020;
pub const XCAN_IXR_RXOK_MASK: u32 = 0x00000010;
pub const XCAN_IXR_TXBFLL_MASK: u32 = 0x00000008;
pub const XCAN_IXR_TXFLL_MASK: u32 = 0x00000004;
pub const XCAN_IXR_TXOK_MASK: u32 = 0x00000002;
pub const XCAN_IXR_ARBLST_MASK: u32 = 0x00000001;

pub const XCAN_IXR_ECC_MASK: u32 = XCAN_IXR_E2BERX_MASK | XCAN_IXR_E1BERX_MASK | XCAN_IXR_E2BETXOL_MASK | XCAN_IXR_E1BETXOL_MASK | XCAN_IXR_E2BETXTL_MASK | XCAN_IXR_E1BETXTL_MASK;

pub const XCAN_IXR_ALL: u32 = XCAN_IXR_WKUP_MASK | XCAN_IXR_SLP_MASK | XCAN_IXR_BSOFF_MASK | XCAN_IXR_ERROR_MASK | XCAN_IXR_RXNEMP_MASK | XCAN_IXR_RXOFLW_MASK | XCAN_IXR_RXUFLW_MASK | XCAN_IXR_RXOK_MASK | XCAN_IXR_TXBFLL_MASK | XCAN_IXR_TXFLL_MASK | XCAN_IXR_TXOK_MASK | XCAN_IXR_ARBLST_MASK;

/// CAN Frame Identifier
pub const XCAN_IDR_ID1_MASK: u32 = 0xFFE00000;
pub const XCAN_IDR_ID1_SHIFT: u32 = 21;
pub const XCAN_IDR_SRR_MASK: u32 = 0x00100000;
pub const XCAN_IDR_SRR_SHIFT: u32 = 20;
pub const XCAN_IDR_IDE_MASK: u32 = 0x00080000;
pub const XCAN_IDR_IDE_SHIFT: u32 = 19;
pub const XCAN_IDR_ID2_MASK: u32 = 0x0007FFFE;
pub const XCAN_IDR_ID2_SHIFT: u32 = 1;
pub const XCAN_IDR_RTR_MASK: u32 = 0x00000001;

/// CAN Frame Data Length Code
pub const XCAN_DLCR_DLC_MASK: u32 = 0xF0000000;
pub const XCAN_DLCR_DLC_SHIFT: u32 = 28;

/// CAN Frame Data Word 1
pub const XCAN_DW1R_DB0_MASK: u32 = 0xFF000000;
pub const XCAN_DW1R_DB0_SHIFT: u32 = 24;
pub const XCAN_DW1R_DB1_MASK: u32 = 0x00FF0000;
pub const XCAN_DW1R_DB1_SHIFT: u32 = 16;
pub const XCAN_DW1R_DB2_MASK: u32 = 0x0000FF00;
pub const XCAN_DW1R_DB2_SHIFT: u32 = 8;
pub const XCAN_DW1R_DB3_MASK: u32 = 0x000000FF;

/// CAN Frame Data Word 2
pub const XCAN_DW2R_DB4_MASK: u32 = 0xFF000000;
pub const XCAN_DW2R_DB4_SHIFT: u32 = 24;
pub const XCAN_DW2R_DB5_MASK: u32 = 0x00FF0000;
pub const XCAN_DW2R_DB5_SHIFT: u32 = 16;
pub const XCAN_DW2R_DB6_MASK: u32 = 0x0000FF00;
pub const XCAN_DW2R_DB6_SHIFT: u32 = 8;
pub const XCAN_DW2R_DB7_MASK: u32 = 0x000000FF;

/// Acceptance Filter Register
pub const XCAN_AFR_UAF4_MASK: u32 = 0x00000008;
pub const XCAN_AFR_UAF3_MASK: u32 = 0x00000004;
pub const XCAN_AFR_UAF2_MASK: u32 = 0x00000002;
pub const XCAN_AFR_UAF1_MASK: u32 = 0x00000001;
pub const XCAN_AFR_UAF_ALL_MASK: u32 = XCAN_AFR_UAF4_MASK | XCAN_AFR_UAF3_MASK | XCAN_AFR_UAF2_MASK | XCAN_AFR_UAF1_MASK;

/// ECC Configuration register
pub const XCAN_ECC_CFG_RST_MASK: u32 = 0x00000007;
pub const XCAN_ECC_CFG_REECRX_MASK: u32 = 0x00000004;
pub const XCAN_ECC_CFG_REECTXOL_MASK: u32 = 0x00000002;
pub const XCAN_ECC_CFG_REECTXTL_MASK: u32 = 0x00000001;
pub const XCAN_ECC_2BIT_SHIFT: u32 = 16;

/// CAN frame length constants
pub const XCAN_MAX_FRAME_SIZE: u32 = 16;

/// Mask for Low 16 bits and High 16 bits
pub const XCAN_MASK_LOW_16BITS: u32 = 0x0000FFFF;
pub const XCAN_MASK_HIGH_16BITS: u32 = 0xFFFF0000;


pub const XCAN_MODE_CONFIG: u8 = 0x00000001;
pub const XCAN_MODE_NORMAL: u8 = 0x00000002;
pub const XCAN_MODE_LOOPBACK: u8 = 0x00000004;
pub const XCAN_MODE_SLEEP: u8 = 0x00000008;

pub const BASE_ADDRESS: u32 = 0xE0008000;
pub const BRPR_BAUD_PRESCALAR: u8 = 29;
pub const BTR_SYNC_JUNP_WIDTH: u8 = 3;
pub const BTR_SECOND_TIMESEGMENT: u8 = 2;
pub const BTR_FIRST_TIMESEGMENT: u8 = 15;

use core::ptr;
use crate::print;
use crate::tecs_print::*;
use itron::task::*;
use itron::time::{duration, Duration, timeout, Timeout};

 /// Function to read a register value
 pub fn x_can_read_reg(base_address: *const u32, reg_offset: u32) -> u32 {
    // let temp = base_address.add(reg_offset as usize / 4);
    // print!("Read from: %tX", temp as u32);
    // delay(duration!(ms: 100)).expect("delay failed");
    unsafe{
        ptr::read_volatile(base_address.add(reg_offset as usize / 4))
    }
 }
 
 /// Function to write a register value
 pub fn x_can_write_reg(base_address: *mut u32, reg_offset: u32, data: u32) {
    // let temp = base_address.add(reg_offset as usize / 4);
    // print!("Write to: %tX", temp as u32);
    // delay(duration!(ms: 100)).expect("delay failed");
    unsafe{
        ptr::write_volatile(base_address.add(reg_offset as usize / 4), data);
    }
 }

 /// Function to create an ID value for CAN frames
pub fn x_can_create_id_value(
    standard_id: u32,
    sub_remote_trans_req: u32,
    id_extension: u32,
    extended_id: u32,
    remote_trans_req: u32,
) -> u32 {
    ((standard_id << XCAN_IDR_ID1_SHIFT) & XCAN_IDR_ID1_MASK)
        | ((sub_remote_trans_req << XCAN_IDR_SRR_SHIFT) & XCAN_IDR_SRR_MASK)
        | ((id_extension << XCAN_IDR_IDE_SHIFT) & XCAN_IDR_IDE_MASK)
        | ((extended_id << XCAN_IDR_ID2_SHIFT) & XCAN_IDR_ID2_MASK)
        | (remote_trans_req & XCAN_IDR_RTR_MASK)
}

/// Function to create a DLC value for CAN frames
pub fn x_can_create_dlc_value(data_length_code: u32) -> u32 {
    (data_length_code << XCAN_DLCR_DLC_SHIFT) & XCAN_DLCR_DLC_MASK
}

pub fn x_can_loopback_setup() {
    if !x_can_enter_mode(XCAN_MODE_CONFIG) {
    }

    while x_can_get_mode() != XCAN_MODE_CONFIG {}

    x_can_set_baud_rate_prescaler();
    x_can_set_bit_timing();

    x_can_interrupt_enable(XCAN_IXR_RXNEMP_MASK);

    if !x_can_enter_mode(XCAN_MODE_LOOPBACK){
    }
    while x_can_get_mode() != XCAN_MODE_LOOPBACK {}
}

pub fn x_can_send(tx_frame: &[u32; 16]) -> bool{

    if x_can_is_tx_fifo_full() {
        return false;
    }

    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_TXFIFO_ID_OFFSET, tx_frame[0]);
    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_TXFIFO_DLC_OFFSET, tx_frame[1]);
    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_TXFIFO_DW1_OFFSET, tx_frame[2]);
    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_TXFIFO_DW2_OFFSET, tx_frame[3]);

    true
}

pub fn x_can_receive(rx_frame: &mut [u32; 16]) -> bool{

    if x_can_is_rx_empty() {
        return false;
    }

    rx_frame[0] = x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_RXFIFO_ID_OFFSET);
    rx_frame[1] = x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_RXFIFO_DLC_OFFSET);
    rx_frame[2] = x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_RXFIFO_DW1_OFFSET);
    rx_frame[3] = x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_RXFIFO_DW2_OFFSET);

    x_can_interrupt_clear(XCAN_IXR_RXNEMP_MASK);

    true
}

pub fn x_can_get_status() -> u32{
    x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_SR_OFFSET)
}

pub fn x_can_get_mode() -> u8{

    let value = x_can_get_status();

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

pub fn x_can_enter_mode(operation_mode: u8)-> bool {

    let current_mode = x_can_get_mode();

    if (current_mode == XCAN_MODE_NORMAL && operation_mode == XCAN_MODE_SLEEP) ||
       (current_mode == XCAN_MODE_SLEEP && operation_mode == XCAN_MODE_NORMAL) {
        
        let value = if operation_mode == XCAN_MODE_SLEEP {
            XCAN_MSR_SLEEP_MASK
        } else {
            0
        };
        
        x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_MSR_OFFSET, value);
    
        return true;
    }

    // Configuration Mode に入る
    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_SRR_OFFSET, 0);

    if x_can_get_mode() != XCAN_MODE_CONFIG {
        return false;
    }

    match operation_mode {
        XCAN_MODE_CONFIG => {}, // 既に Configuration Mode のため何もしない
        XCAN_MODE_SLEEP => {
            x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_MSR_OFFSET, XCAN_MSR_SLEEP_MASK);
            x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_SRR_OFFSET, XCAN_SRR_CEN_MASK);
        }
        XCAN_MODE_NORMAL => {
            x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_MSR_OFFSET, 0);
            x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_SRR_OFFSET, XCAN_SRR_CEN_MASK);
        }
        XCAN_MODE_LOOPBACK => {
            x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_MSR_OFFSET, XCAN_MSR_LBACK_MASK);
            x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_SRR_OFFSET, XCAN_SRR_CEN_MASK);
        }
        _ => unreachable!(),
    }

    true
}

pub fn x_can_set_baud_rate_prescaler() -> bool{

    if x_can_get_mode() != XCAN_MODE_CONFIG {
        return false;
    }

    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_BRPR_OFFSET, BRPR_BAUD_PRESCALAR as u32);

    true
}

pub fn x_can_set_bit_timing() -> bool{

    if BTR_SYNC_JUNP_WIDTH > 3 || BTR_SECOND_TIMESEGMENT > 7 || BTR_FIRST_TIMESEGMENT > 15 {
        return false;
    }

    if x_can_get_mode() != XCAN_MODE_CONFIG {
        return false;
    }

    let mut value = (BTR_FIRST_TIMESEGMENT as u32) & XCAN_BTR_TS1_MASK;
    value |= ((BTR_SECOND_TIMESEGMENT as u32) << XCAN_BTR_TS2_SHIFT) & XCAN_BTR_TS2_MASK;
    value |= ((BTR_SYNC_JUNP_WIDTH as u32) << XCAN_BTR_SJW_SHIFT) & XCAN_BTR_SJW_MASK;

    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_BTR_OFFSET, value);

    true
}

pub fn x_can_is_tx_fifo_full() -> bool{
    (x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_SR_OFFSET) & XCAN_SR_TXFLL_MASK) != 0
}

pub fn x_can_is_rx_empty() -> bool{
    (x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_ISR_OFFSET) & XCAN_IXR_RXNEMP_MASK) == 0
}

pub fn x_can_interrupt_enable(mask: u32){
    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_IER_OFFSET, mask);
}

pub fn x_can_interrupt_clear(mask: u32){
    let mut intr_value = x_can_interrupt_status();
    intr_value &= mask;
    x_can_write_reg(BASE_ADDRESS as *mut u32, XCAN_ICR_OFFSET, intr_value);
}

pub fn x_can_interrupt_status() -> u32{
    x_can_read_reg(BASE_ADDRESS as *const u32, XCAN_ISR_OFFSET)
}