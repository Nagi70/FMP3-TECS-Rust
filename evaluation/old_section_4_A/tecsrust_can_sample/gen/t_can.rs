pub struct TCan{
	base_address: u32,
	message_id: u32,
	frame_data_length: u32,
	brpr_baud_prescalar: u8,
	btr_sync_junp_width: u8,
	btr_second_timesegment: u8,
	btr_first_timesegment: u8,
}

pub struct ECanForTCan<'a>{
	pub cell: &'a TCan,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_CAN: TCan = TCan {
	base_address: 0xE0008000,
	message_id: 1024,
	frame_data_length: 8,
	brpr_baud_prescalar: 29,
	btr_sync_junp_width: 3,
	btr_second_timesegment: 2,
	btr_first_timesegment: 15,
};

#[link_section = ".rodata"]
pub static ECANFORRPROCESSOR1SYMMETRIC_CAN: ECanForTCan = ECanForTCan {
	cell: &RPROCESSOR1SYMMETRIC_CAN,
};

impl<> TCan {
	pub fn get_cell_ref(&'static self) -> (&'static u32, &'static u32, &'static u32, &'static u8, &'static u8, &'static u8, &'static u8) {
		(
			&self.base_address,
			&self.message_id,
			&self.frame_data_length,
			&self.brpr_baud_prescalar,
			&self.btr_sync_junp_width,
			&self.btr_second_timesegment,
			&self.btr_first_timesegment
		)
	}
}
