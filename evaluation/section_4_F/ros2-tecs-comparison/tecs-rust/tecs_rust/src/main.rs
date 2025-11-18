mod tecs_celltype;
mod tecs_signature;
mod tecs_impl;
mod tecs_global;

use tecs_celltype::t_task_rs::*;
use tecs_signature::s_task_body::*;

fn main() {
    println!("tecs_rust: use cargo run --bin imu_corrector or --bin vehicle_velocity_converter to run individual tasks.");
}