use crate::tecs_global::*;
use crate::tecs_celltype::t_can::*;
use crate::tecs_signature::s_can_measure_for_opt::*;
impl SCanMeasureForOpt for ECanForTCan{

	fn loopback_setup(&self) {
		let lg = self.cell.get_cell_ref();

	}
	fn send(&self, tx_frame: &[u32]) -> Result<(),()>{
		let lg = self.cell.get_cell_ref();

	}
	fn receive(&self, rx_frame: &mut [u32]) -> Result<(),()>{
		let lg = self.cell.get_cell_ref();

	}
}

