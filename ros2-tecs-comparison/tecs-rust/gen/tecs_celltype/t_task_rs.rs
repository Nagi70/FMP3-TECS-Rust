use itron::task::TaskRef;
use core::num::NonZeroI32;
use crate::kernel_cfg::*;
use crate::tecs_global::*;
use crate::tecs_signature::s_task_body::*;
use crate::tecs_celltype::{t_imu_corrector::*, t_vehicle_velocity_converter::*};

pub struct TTaskRs<T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'static T,
	task_ref: itron::task::TaskRef<'static>,
}

pub struct LockGuardForTTaskRs<'a, T>
where
	T: STaskBody + 'static,
{
	pub c_task_body: &'a T,
	pub task_ref: &'a itron::task::TaskRef<'static>,
}

#[unsafe(link_section = ".rodata")]
pub static IMUCORRECTOR: TTaskRs<ETaskbodyForTImuCorrector> = TTaskRs {
	c_task_body: &ETASKBODYFORIMUCORRECTORBODY,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_$id$).unwrap())},
};

#[unsafe(link_section = ".rodata")]
pub static VEHICLEVELOCITYCONVERTER: TTaskRs<ETaskbodyForTVehicleVelocityConverter> = TTaskRs {
	c_task_body: &ETASKBODYFORVEHICLEVELOCITYCONVERTERBODY,
	task_ref: unsafe{itron::task::TaskRef::from_raw_nonnull(NonZeroI32::new(TSKID_$id$).unwrap())},
};

