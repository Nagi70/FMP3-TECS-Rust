use crate::tecs_global::*;
pub trait STf {
	fn transform_covariance(&self, src: &[f64])-> Vec<f64>;
	fn quat_normalize(&self, x: &f64, y: &f64, z: &f64, w: &f64)-> (f64, f64, f64, f64);
	fn quat_mul(&self, a: &(f64, f64, f64, f64), b: &(f64, f64, f64, f64))-> (f64, f64, f64, f64);
	fn rotate_vector_by_quat(&self, v: &r2r::geometry_msgs::msg::Vector3)-> r2r::geometry_msgs::msg::Vector3;
}
