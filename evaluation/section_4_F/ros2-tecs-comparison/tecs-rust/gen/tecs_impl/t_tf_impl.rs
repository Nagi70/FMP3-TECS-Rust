use crate::tecs_global::*;
use crate::tecs_celltype::t_tf::*;
use crate::tecs_signature::s_tf::*;
impl STf for ETfForTTf{

	fn transform_covariance(&self, src: &[f64]) -> heapless::Vec<f64, 9>{
		let lg = self.cell.get_cell_ref();

	}
	fn quat_normalize(&self, x: &f64, y: &f64, z: &f64, w: &f64) -> (f64, f64, f64, f64){
		let lg = self.cell.get_cell_ref();

	}
	fn quat_mul(&self, a: &(f64, f64, f64, f64), b: &(f64, f64, f64, f64)) -> (f64, f64, f64, f64){
		let lg = self.cell.get_cell_ref();

	}
	fn rotate_vector_by_quat(&self, v: &r2r::geometry_msgs::msg::Vector3) -> r2r::geometry_msgs::msg::Vector3{
		let lg = self.cell.get_cell_ref();

	}
}

