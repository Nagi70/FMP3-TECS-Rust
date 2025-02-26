use crate::{t_button::*, s_button::*};

impl SButton for EButtonForTButton<'_>{

	fn is_pushed(&'static self) -> bool{
		let (data_1_ro, gpio_offset) = self.cell.get_cell_ref();

	}
}

