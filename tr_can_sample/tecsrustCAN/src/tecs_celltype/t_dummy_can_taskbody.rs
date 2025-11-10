use crate::tecs_global::*;
use crate::tecs_signature::s_can_measure_for_opt::*;
use crate::tecs_celltype::t_can::*;
pub struct TDummyCanTaskbody<T>
where
	T: SCanMeasureForOpt + 'static,
{
	c_can: &'static T,
}

pub struct ETaskbodyForTDummyCanTaskbody {
	pub cell: &'static TDummyCanTaskbody<ECanForTCan>,
}

pub struct LockGuardForTDummyCanTaskbody<'a, T>
where
	T: SCanMeasureForOpt + 'static,
{
	pub c_can: &'a T,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_CANBODY: TDummyCanTaskbody<ECanForTCan> = TDummyCanTaskbody {
	c_can: &ECANFORRPROCESSOR1SYMMETRIC_CAN,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_CANBODY: ETaskbodyForTDummyCanTaskbody = ETaskbodyForTDummyCanTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_CANBODY,
};

impl<T: SCanMeasureForOpt> TDummyCanTaskbody<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTDummyCanTaskbody<'_, T> {
		LockGuardForTDummyCanTaskbody {
			c_can: self.c_can,
		}
	}
}
