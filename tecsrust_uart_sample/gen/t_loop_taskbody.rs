pub struct TLoopTaskbody{
}

pub struct ETaskbodyForTLoopTaskbody<'a>{
	pub cell: &'a TLoopTaskbody,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_LOOPTASKBODY: TLoopTaskbody = TLoopTaskbody {
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_LOOPTASKBODY: ETaskbodyForTLoopTaskbody = ETaskbodyForTLoopTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_LOOPTASKBODY,
};

