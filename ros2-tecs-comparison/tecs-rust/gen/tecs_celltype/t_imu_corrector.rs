use crate::tecs_global::*;
use crate::tecs_signature::s_tf::*;
use crate::tecs_celltype::t_tf::*;
pub struct TImuCorrector<T>
where
	T: STf + 'static,
{
	c_tf: &'static T,
	output_frame: &'static str,
	angular_velocity_offset_x: f64,
	angular_velocity_offset_y: f64,
	angular_velocity_offset_z: f64,
	angular_velocity_stddev_xx: f64,
	angular_velocity_stddev_yy: f64,
	angular_velocity_stddev_zz: f64,
	accel_stddev: f64,
}

pub struct ETaskbodyForTImuCorrector {
	pub cell: &'static TImuCorrector<ETfForTTf>,
}

pub struct LockGuardForTImuCorrector<'a, T>
where
	T: STf + 'static,
{
	pub c_tf: &'a T,
	pub output_frame: &'a &'static str,
	pub angular_velocity_offset_x: &'a f64,
	pub angular_velocity_offset_y: &'a f64,
	pub angular_velocity_offset_z: &'a f64,
	pub angular_velocity_stddev_xx: &'a f64,
	pub angular_velocity_stddev_yy: &'a f64,
	pub angular_velocity_stddev_zz: &'a f64,
	pub accel_stddev: &'a f64,
}

#[unsafe(link_section = ".rodata")]
static IMUCORRECTORBODY: TImuCorrector<ETfForTTf> = TImuCorrector {
	c_tf: &ETFFORTF,
	output_frame: "base_link",
	angular_velocity_offset_x: 0.0,
	angular_velocity_offset_y: 0.0,
	angular_velocity_offset_z: 0.0,
	angular_velocity_stddev_xx: 0.03,
	angular_velocity_stddev_yy: 0.03,
	angular_velocity_stddev_zz: 0.03,
	accel_stddev: 10000.0,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORIMUCORRECTORBODY: ETaskbodyForTImuCorrector = ETaskbodyForTImuCorrector {
	cell: &IMUCORRECTORBODY,
};

impl<T: STf> TImuCorrector<T> {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTImuCorrector<'_, T> {
		LockGuardForTImuCorrector {
			c_tf: self.c_tf,
			output_frame: &self.output_frame,
			angular_velocity_offset_x: &self.angular_velocity_offset_x,
			angular_velocity_offset_y: &self.angular_velocity_offset_y,
			angular_velocity_offset_z: &self.angular_velocity_offset_z,
			angular_velocity_stddev_xx: &self.angular_velocity_stddev_xx,
			angular_velocity_stddev_yy: &self.angular_velocity_stddev_yy,
			angular_velocity_stddev_zz: &self.angular_velocity_stddev_zz,
			accel_stddev: &self.accel_stddev,
		}
	}
}
