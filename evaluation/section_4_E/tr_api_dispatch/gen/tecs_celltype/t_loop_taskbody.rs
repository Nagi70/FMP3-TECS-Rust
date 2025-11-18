use crate::tecs_global::*;
pub struct TLoopTaskbody{
}

pub struct ETaskbodyForTLoopTaskbody {
	pub cell: &'static TLoopTaskbody,
}

#[unsafe(link_section = ".rodata")]
static RPROCESSOR2SYMMETRIC_LOOPTASKBODY: TLoopTaskbody = TLoopTaskbody {
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_LOOPTASKBODY: ETaskbodyForTLoopTaskbody = ETaskbodyForTLoopTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_LOOPTASKBODY,
};

