use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::{t_imu_corrector::*, t_vehicle_velocity_converter::*};

pub struct TTaskRs<T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'static T,
}

pub struct LockGuardForTTaskRs<'a, T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'a T,
}

#[unsafe(link_section = ".rodata")]
pub static IMUCORRECTOR: TTaskRs<ETaskbodyForTImuCorrector> = TTaskRs {
	c_task_body: &ETASKBODYFORIMUCORRECTORBODY,
};

#[unsafe(link_section = ".rodata")]
pub static VEHICLEVELOCITYCONVERTER: TTaskRs<ETaskbodyForTVehicleVelocityConverter> = TTaskRs {
	c_task_body: &ETASKBODYFORVEHICLEVELOCITYCONVERTERBODY,
};

