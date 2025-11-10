use crate::tecs_global::*;
use crate::tecs_signature::s_can_measure_for_opt::*;
use crate::tecs_celltype::t_can::*;
pub struct TCanTaskbody<T>
where
	T: SCanMeasureForOpt + 'static,
{
	c_can: &'static T,
}

pub struct ETaskbodyForTCanTaskbody {
	pub cell: &'static TCanTaskbody<ECanForTCan>,
}

pub struct LockGuardForTCanTaskbody<'a, T>
where
	T: SCanMeasureForOpt + 'static,
{
	pub c_can: &'a T,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR1SYMMETRIC_CANTASKBODY: TCanTaskbody<ECanForTCan> = TCanTaskbody {
	c_can: &ECANFORRPROCESSOR1SYMMETRIC_CAN,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR1SYMMETRIC_CANTASKBODY: ETaskbodyForTCanTaskbody = ETaskbodyForTCanTaskbody {
	cell: &RPROCESSOR1SYMMETRIC_CANTASKBODY,
};

impl<T: SCanMeasureForOpt> TCanTaskbody<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTCanTaskbody<'_, T> {
		LockGuardForTCanTaskbody {
			c_can: self.c_can,
		}
	}
}
