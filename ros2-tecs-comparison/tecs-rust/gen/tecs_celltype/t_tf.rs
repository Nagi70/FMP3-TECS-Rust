use crate::tecs_global::*;
pub struct TTf{
	x: f64,
	y: f64,
	sin_half_yaw: f64,
	cos_half_yaw: f64,
}

pub struct ETfForTTf {
	pub cell: &'static TTf,
}

pub struct LockGuardForTTf<'a>{
	pub x: &'a f64,
	pub y: &'a f64,
	pub sin_half_yaw: &'a f64,
	pub cos_half_yaw: &'a f64,
}

#[unsafe(link_section = ".rodata")]
static TF: TTf = TTf {
	x: 0.0,
	y: 0.0,
	sin_half_yaw: 0.04997917,
	cos_half_yaw: 0.99875026,
};

#[unsafe(link_section = ".rodata")]
pub static ETFFORTF: ETfForTTf = ETfForTTf {
	cell: &TF,
};

impl TTf {
	#[inline]
	pub fn get_cell_ref(&'static self) -> LockGuardForTTf<'_> {
		LockGuardForTTf {
			x: &self.x,
			y: &self.y,
			sin_half_yaw: &self.sin_half_yaw,
			cos_half_yaw: &self.cos_half_yaw,
		}
	}
}
