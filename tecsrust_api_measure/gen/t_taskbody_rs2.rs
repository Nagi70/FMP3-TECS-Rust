pub struct TTaskbodyRs2
{
}

pub struct ETaskbodyForTTaskbodyRs2<'a>{
	pub cell: &'a TTaskbodyRs2,
}

#[link_section = ".rodata"]
pub static RPROCESSORALLMIG_TASKBODY1_2: TTaskbodyRs2 = TTaskbodyRs2 {
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSORALLMIG_TASKBODY1_2: ETaskbodyForTTaskbodyRs2 = ETaskbodyForTTaskbodyRs2 {
	cell: &RPROCESSORALLMIG_TASKBODY1_2,
};

