use crate::tecs_global::*;
pub struct TVehicleVelocityConverter{
	frame_id: &'static str,
	velocity_stddev_xx: f64,
	angular_velocity_stddev_zz: f64,
	speed_scale_factor: f64,
}

pub struct ETaskbodyForTVehicleVelocityConverter {
	pub cell: &'static TVehicleVelocityConverter,
}

pub struct LockGuardForTVehicleVelocityConverter<'a>{
	pub frame_id: &'a &'static str,
	pub velocity_stddev_xx: &'a f64,
	pub angular_velocity_stddev_zz: &'a f64,
	pub speed_scale_factor: &'a f64,
}

#[unsafe(link_section = ".rodata")]
static VEHICLEVELOCITYCONVERTERBODY: TVehicleVelocityConverter = TVehicleVelocityConverter {
	frame_id: "base_link",
	velocity_stddev_xx: 0.2,
	angular_velocity_stddev_zz: 0.1,
	speed_scale_factor: 1.0,
};

#[unsafe(link_section = ".rodata")]
pub static ETASKBODYFORVEHICLEVELOCITYCONVERTERBODY: ETaskbodyForTVehicleVelocityConverter = ETaskbodyForTVehicleVelocityConverter {
	cell: &VEHICLEVELOCITYCONVERTERBODY,
};

impl TVehicleVelocityConverter {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTVehicleVelocityConverter<'_> {
		LockGuardForTVehicleVelocityConverter {
			frame_id: &self.frame_id,
			velocity_stddev_xx: &self.velocity_stddev_xx,
			angular_velocity_stddev_zz: &self.angular_velocity_stddev_zz,
			speed_scale_factor: &self.speed_scale_factor,
		}
	}
}
