use crate::{s_can_measure::*, t_can::*};

pub struct TCanTaskbody<'a, T>where
	T: SCanMeasure,
{
	c_can: &'a T,
}

pub struct ETaskbodyForTCanTaskbody<'a>{
	pub cell: &'a TCanTaskbody<'a, ECanForTCan<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR1SYMMETRIC_CANTASKBODY: TCanTaskbody<ECanForTCan> = TCanTaskbody {
	c_can: &ECANFORRPROCESSOR1SYMMETRIC_CAN,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_CANTASKBODY: ETaskbodyForTCanTaskbody = ETaskbodyForTCanTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_CANTASKBODY,
};

impl<T: SCanMeasure> TCanTaskbody<'_, T> {
	pub fn get_cell_ref(&'static self) -> &'static T {
		self.c_can
	}
}
