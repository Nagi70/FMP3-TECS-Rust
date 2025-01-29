pub struct TPrint
{
}

pub struct EPrintForTPrint<'a>{
	pub cell: &'a TPrint,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_PRINT2_1: TPrint = TPrint {
};

#[link_section = ".rodata"]
pub static EPRINTFORRPROCESSOR2SYMMETRIC_PRINT2_1: EPrintForTPrint = EPrintForTPrint {
	cell: &RPROCESSOR2SYMMETRIC_PRINT2_1,
};

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_PRINT2_2: TPrint = TPrint {
};

#[link_section = ".rodata"]
pub static EPRINTFORRPROCESSOR2SYMMETRIC_PRINT2_2: EPrintForTPrint = EPrintForTPrint {
	cell: &RPROCESSOR2SYMMETRIC_PRINT2_2,
};

