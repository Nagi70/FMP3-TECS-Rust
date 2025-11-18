use crate::tecs_global::*;
use crate::tecs_celltype::t_tf::*;
use crate::tecs_signature::s_tf::*;
impl STf for ETfForTTf{

	fn transform_covariance(&self, src: &[f64]) -> Vec<f64>{
		let lg = self.cell.get_cell_ref();

		let x_x = *src.get(0).unwrap_or(&0.0);
		let y_y = *src.get(4).unwrap_or(&0.0);
		let z_z = *src.get(8).unwrap_or(&0.0);
		let max_cov = x_x.max(y_y).max(z_z);
		let mut out = vec![0.0; 9];
		out[0] = max_cov;
		out[4] = max_cov;
		out[8] = max_cov;
		out
	}
	fn quat_normalize(&self, x: &f64, y: &f64, z: &f64, w: &f64) -> (f64, f64, f64, f64){
		let lg = self.cell.get_cell_ref();

		let n = (*x * *x + *y * *y + *z * *z + *w * *w).sqrt();
		if n > 0.0 {
			(*x / n, *y / n, *z / n, *w / n)
		} else {
			// fallback to identity
			(0.0, 0.0, 0.0, 1.0)
		}
	}
	fn quat_mul(&self, a: &(f64, f64, f64, f64), b: &(f64, f64, f64, f64)) -> (f64, f64, f64, f64){
		let lg = self.cell.get_cell_ref();

		let (ax, ay, az, aw) = *a;
		let (bx, by, bz, bw) = *b;
		let x = aw * bx + ax * bw + ay * bz - az * by;
		let y = aw * by - ax * bz + ay * bw + az * bx;
		let z = aw * bz + ax * by - ay * bx + az * bw;
		let w = aw * bw - ax * bx - ay * by - az * bz;
		(x, y, z, w)
	}
	fn rotate_vector_by_quat(&self, v: &r2r::geometry_msgs::msg::Vector3) -> r2r::geometry_msgs::msg::Vector3{
		let lg = self.cell.get_cell_ref();

		let (qx, qy, qz, qw) = self.quat_normalize(lg.x, lg.y, lg.sin_half_yaw, lg.cos_half_yaw);
		let vq = (v.x, v.y, v.z, 0.0);
		let qv = self.quat_mul(&(qx, qy, qz, qw), &vq);
		let q_conj = (-qx, -qy, -qz, qw);
		let res = self.quat_mul(&qv, &q_conj);
		r2r::geometry_msgs::msg::Vector3 { x: res.0, y: res.1, z: res.2 }
	}
}

