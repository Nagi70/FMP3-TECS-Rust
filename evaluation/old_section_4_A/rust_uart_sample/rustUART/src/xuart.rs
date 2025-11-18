pub const XUART_CR_OFFSET: u32 = 0x00;
pub const XUART_MR_OFFSET: u32 = 0x04;
pub const XUART_IER_OFFSET: u32 = 0x08;
pub const XUART_IDR_OFFSET: u32 = 0x0c;
pub const XUART_ISR_OFFSET: u32 = 0x14;
pub const XUART_BAUDGEN_OFFSET: u32 = 0x18;
pub const XUART_RXTOUT_OFFSET: u32 = 0x1c;
pub const XUART_RXWM_OFFSET: u32 = 0x20;
pub const XUART_SR_OFFSET: u32 = 0x2c;
pub const XUART_FIFO_OFFSET: u32 = 0x30;
pub const XUART_BAUDDIV_OFFSET: u32 = 0x34;

pub const XUART_CR_STOPPRK: u32 = 0x0100;
pub const XUART_CR_TX_DIS: u32 = 0x0020;
pub const XUART_CR_TX_EN: u32 = 0x0010;
pub const XUART_CR_RX_DIS: u32 = 0x0008;
pub const XUART_CR_RX_EN: u32 = 0x0004;
pub const XUART_CR_TXRST: u32 = 0x0002;
pub const XUART_CR_RXRST: u32 = 0x0001;

pub const XUART_MR_STOPBIT_1: u32 = 0x0000;
pub const XUART_MR_PARITY_NONE: u32 = 0x0020;
pub const XUART_MR_CHARLEN_8: u32 = 0x0000;
pub const XUART_MR_CLKSEL: u32 = 0x0001;
pub const XUART_MR_CCLK: u32 = 0x0400;


pub const XUART_IXR_TXEMPTY: u32 = 0x0008;
pub const XUART_IXR_RXTRIG: u32 = 0x0001;
pub const XUART_IXR_ALL: u32 = 0x1fff;

pub const XUART_SR_TXFULL: u32 = 0x0010;
pub const XUART_SR_TXEMPTY: u32 = 0x0008;
pub const XUART_SR_RXEMPTY: u32 = 0x0002;

pub const XUART_BAUDGEN_115K: u32 = 0x7c;
pub const XUART_BAUDDIV_115K: u32 = 0x06;

use core::ptr::{write_volatile, read_volatile};

pub struct Xuart {
    base_address: u32,
}

impl Xuart {
    pub fn open(base_address: u32) -> Self {
        Self { base_address }
    }

    pub fn set_up(&self, mode: u32, baudgen: u32, bauddiv: u32) {
		unsafe{
			write_volatile((self.base_address + XUART_IDR_OFFSET) as *mut u32, XUART_IXR_ALL);

			write_volatile((self.base_address + XUART_ISR_OFFSET) as *mut u32, read_volatile((self.base_address + XUART_ISR_OFFSET) as *const u32));

			write_volatile((self.base_address + XUART_CR_OFFSET) as *mut u32,
						   XUART_CR_TXRST | XUART_CR_RXRST | XUART_CR_TX_DIS | XUART_CR_RX_DIS);
			
			write_volatile((self.base_address + XUART_BAUDGEN_OFFSET) as *mut u32, baudgen);
			write_volatile((self.base_address + XUART_BAUDDIV_OFFSET) as *mut u32, bauddiv);

			write_volatile((self.base_address + XUART_MR_OFFSET) as *mut u32, mode);

			write_volatile((self.base_address + XUART_RXWM_OFFSET) as *mut u32, 0x01);
			
			write_volatile((self.base_address + XUART_RXTOUT_OFFSET) as *mut u32, 0x10);

			write_volatile((self.base_address + XUART_IER_OFFSET) as *mut u32, XUART_IXR_TXEMPTY);
			write_volatile((self.base_address + XUART_IER_OFFSET) as *mut u32, XUART_IXR_RXTRIG);

			write_volatile((self.base_address + XUART_CR_OFFSET) as *mut u32, XUART_CR_TX_EN | XUART_CR_RX_EN | XUART_CR_STOPPRK);
		}
    }

    pub fn put_char(&self, c: u8) -> bool {
        unsafe {
            if((read_volatile((self.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0x00)
            {
                write_volatile((self.base_address + XUART_FIFO_OFFSET) as *mut u32, c as u32);
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn get_char(&self, c: &mut u8) -> bool {
        unsafe {
            if((read_volatile((self.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_RXEMPTY) == 0x00)
            {
                *c = read_volatile((self.base_address + XUART_FIFO_OFFSET) as *const u32) as u8;
                return true;
            }
        }

        return false;
    }

    pub fn handler(&self) {
		// 受信
		unsafe{
			// if (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_RXTRIG) == 1 {
			// 	while (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_SR_RXEMPTY) == 0 {
			// 		c_xuart_main.ready_receive();
			// 	}
			// }

			if (read_volatile((self.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_RXEMPTY) == 0 {
				self.ready_receive();
			}

			write_volatile((self.base_address + XUART_ISR_OFFSET) as *mut u32, (read_volatile((self.base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_RXTRIG));
		}

		// 送信
		unsafe{
			// if (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_TXEMPTY) == 1 {
			// 	while (read_volatile((*base_address + XUART_ISR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0 {
			// 		c_xuart_main.ready_send();
			// 	}
			// }

			if (read_volatile((self.base_address + XUART_SR_OFFSET) as *const u32) & XUART_SR_TXFULL) == 0 {
				self.ready_send();
			}

			write_volatile((self.base_address + XUART_ISR_OFFSET) as *mut u32, (read_volatile((self.base_address + XUART_ISR_OFFSET) as *const u32) & XUART_IXR_TXEMPTY));
		}
    }

    pub fn ready_send(&self) {
    }

    pub fn ready_receive(&self) {
        let mut c: u8 = 0;
        let result = self.get_char(&mut c);
        if result {
            // c_xuart.put_char(&c);
            let data: itron::dataqueue::DataElement = c.into();
            crate::c_dataqueue.send_forced(data);
        }
    }
}
