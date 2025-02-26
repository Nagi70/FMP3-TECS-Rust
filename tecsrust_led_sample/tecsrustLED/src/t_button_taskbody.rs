use crate::{s_button::*, t_button::*};

pub struct TButtonTaskbody<'a, T, U>
where
	T: SButton,
	U: SButton,
{
	c_button1: &'a T,
	c_button2: &'a U,
}

pub struct ETaskbodyForTButtonTaskbody<'a>{
	pub cell: &'a TButtonTaskbody<'a, EButtonForTButton<'a>, EButtonForTButton<'a>>,
}

#[link_section = ".rodata"]
pub static RPROCESSOR2SYMMETRIC_BUTTONTASKBODY: TButtonTaskbody<EButtonForTButton, EButtonForTButton> = TButtonTaskbody {
	c_button1: &EBUTTONFORRPROCESSOR2SYMMETRIC_BUTTON4,
	c_button2: &EBUTTONFORRPROCESSOR2SYMMETRIC_BUTTON5,
};

#[link_section = ".rodata"]
pub static ETASKBODYFORRPROCESSOR2SYMMETRIC_BUTTONTASKBODY: ETaskbodyForTButtonTaskbody = ETaskbodyForTButtonTaskbody {
	cell: &RPROCESSOR2SYMMETRIC_BUTTONTASKBODY,
};

impl<T: SButton, U: SButton> TButtonTaskbody<'_, T, U> {
	pub fn get_cell_ref(&'static self) -> (&'static T, &'static U) {
		(
			self.c_button1,
			self.c_button2
		)
	}
}
