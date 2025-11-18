use futures::{executor::LocalPool, task::LocalSpawnExt};
use r2r::QosProfile;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

// Port of Autoware's imu_corrector callback into Rust with TF-like rotation:
// - Subscribe: sensor_msgs/Imu on "input"
// - Publish: sensor_msgs/Imu on "output" after applying offsets and simple covariance handling
// - TF: apply a dummy quaternion rotation (configurable yaw) instead of identity

fn transform_covariance(src: &[f64]) -> Vec<f64> {
    // src is expected length 9 (row-major 3x3). Compute max diag and put it on the diagonal.
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

fn quat_normalize(mut x: f64, mut y: f64, mut z: f64, mut w: f64) -> (f64, f64, f64, f64) {
    let n = (x * x + y * y + z * z + w * w).sqrt();
    if n > 0.0 {
        x /= n; y /= n; z /= n; w /= n;
    } else {
        // fallback to identity
        x = 0.0; y = 0.0; z = 0.0; w = 1.0;
    }
    (x, y, z, w)
}

fn quat_mul(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let (ax, ay, az, aw) = a;
    let (bx, by, bz, bw) = b;
    let x = aw * bx + ax * bw + ay * bz - az * by;
    let y = aw * by - ax * bz + ay * bw + az * bx;
    let z = aw * bz + ax * by - ay * bx + az * bw;
    let w = aw * bw - ax * bx - ay * by - az * bz;
    (x, y, z, w)
}

fn rotate_vector_by_quat(v: &r2r::geometry_msgs::msg::Vector3, q: (f64, f64, f64, f64)) -> r2r::geometry_msgs::msg::Vector3 {
    // v' = q * (v,0) * conj(q)
    let (qx, qy, qz, qw) = quat_normalize(q.0, q.1, q.2, q.3);
    let vq = (v.x, v.y, v.z, 0.0);
    let qv = quat_mul((qx, qy, qz, qw), vq);
    let q_conj = (-qx, -qy, -qz, qw);
    let res = quat_mul(qv, q_conj);
    r2r::geometry_msgs::msg::Vector3 { x: res.0, y: res.1, z: res.2 }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "imu_corrector", "")?;

    // Parameters (with defaults similar to C++)
    let output_frame: String = node.get_parameter::<String>("base_link").unwrap_or_else(|_| "base_link".to_string());
    let offset_x: f64 = node.get_parameter::<f64>("angular_velocity_offset_x").unwrap_or(0.0);
    let offset_y: f64 = node.get_parameter::<f64>("angular_velocity_offset_y").unwrap_or(0.0);
    let offset_z: f64 = node.get_parameter::<f64>("angular_velocity_offset_z").unwrap_or(0.0);
    let stddev_xx: f64 = node.get_parameter::<f64>("angular_velocity_stddev_xx").unwrap_or(0.03);
    let stddev_yy: f64 = node.get_parameter::<f64>("angular_velocity_stddev_yy").unwrap_or(0.03);
    let stddev_zz: f64 = node.get_parameter::<f64>("angular_velocity_stddev_zz").unwrap_or(0.03);
    let accel_stddev: f64 = node.get_parameter::<f64>("acceleration_stddev").unwrap_or(10000.0);
    // Dummy TF yaw [rad] (rotation around Z axis). Set non-zero default to show effect.
    let dummy_tf_yaw: f64 = node.get_parameter::<f64>("dummy_tf_yaw_rad").unwrap_or(0.1);

    // Publisher (IMU only; no bias/scale inputs). Use timer + fixed input instead of subscribing.
    let pub_out = node.create_publisher::<r2r::sensor_msgs::msg::Imu>("output", QosProfile::default())?;

    let input = r2r::sensor_msgs::msg::Imu {
        header: r2r::std_msgs::msg::Header {
            stamp: {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_else(|_| std::time::Duration::from_secs(0));
                r2r::builtin_interfaces::msg::Time {
                    sec: now.as_secs() as i32,
                    nanosec: now.subsec_nanos(),
                }
            },
            frame_id: "imu_link".to_string(),
        },
        angular_velocity: r2r::geometry_msgs::msg::Vector3 {
            x: 0.1,
            y: 0.1,
            z: 0.1,
        },
        linear_acceleration: r2r::geometry_msgs::msg::Vector3 {
            x: 0.1,
            y: 0.1,
            z: 0.1,
        },
        ..Default::default()
    };

    // Timer (e.g., 100 Hz)
    let mut timer = node.create_wall_timer(Duration::from_millis(10))?;

    // Optional parameter to stop after N iterations (default: 1000)
    let max_iter_param: f64 = node.get_parameter::<f64>("max_iterations").unwrap_or(2_000_000.0);
    let max_iterations: u64 = if max_iter_param.is_finite() && max_iter_param > 0.0 {
        max_iter_param as u64
    } else { 1000 };
    let done = Arc::new(AtomicBool::new(false));

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    // ===================== 計測セットアップ =====================
    // ウォームアップ回数（統計に含めない）。必要なら ROS パラメータ化可能。
    let warmup_param: f64 = node.get_parameter::<f64>("warmup_iterations").unwrap_or(10_000.0);
    let warmup_iterations: u64 = if warmup_param.is_finite() && warmup_param >= 0.0 {
        warmup_param as u64
    } else { 10_000 };

    // elapsed() 呼び出しオーバーヘッドの概算（サンプル数は大きめに）
    const OVERHEAD_SAMPLES: usize = 2_000_000;
    let overhead_reference = std::time::Instant::now();
    let overhead_start = std::time::Instant::now();
    for _ in 0..OVERHEAD_SAMPLES { let _ = overhead_reference.elapsed(); }
    let overhead_total = overhead_start.elapsed();
    let overhead_per_call_ns = overhead_total.as_nanos() as f64 / OVERHEAD_SAMPLES as f64;
    println!(
        "[imu_corrector][bench] elapsed() overhead ≈ {:.2} ns ({} samples)",
        overhead_per_call_ns, OVERHEAD_SAMPLES
    );

    // Main IMU handler
    let output_frame_cloned = output_frame.clone();
    let done_for_task = done.clone();
    let overhead_per_call_ns_cl = overhead_per_call_ns;
    let warmup_iterations_cl = warmup_iterations;
    let max_iterations_cl = max_iterations;
    spawner.spawn_local(async move {
        let pub_out = pub_out; // move into task
        // 1ティックだけ待つ
        let _ = timer.tick().await;

        // 統計用（ストリーミング集計: Welford）
        let mut n: u64 = 0;
        let mut mean_ns: f64 = 0.0;
        let mut m2: f64 = 0.0;
        let mut min_ns: u128 = u128::MAX;
        let mut max_ns_v: u128 = 0;

        // 近似分位点用リザーバサンプリング
        const RESERVOIR_K: usize = 65_536;
        let mut reservoir: Vec<u128> = Vec::with_capacity(RESERVOIR_K);
        let mut seen: u64 = 0;
        // 簡易LCG RNG
        let mut rng_state: u64 = {
            let t = std::time::Instant::now().elapsed().as_nanos() as u64;
            0xA24BAED4963EE407u64 ^ t
        };
        let mut lcg_next = |state: &mut u64| -> u64 {
            *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            *state
        };

        // ダミーTFの事前計算
        let half = 0.5 * dummy_tf_yaw;
        let q = (0.0, 0.0, half.sin(), half.cos());

        // 最終的に1回だけPublishするメッセージ
        let mut pub_msg = r2r::sensor_msgs::msg::Imu::default();

        for iter in 0..(warmup_iterations_cl + max_iterations_cl) {
            // 計測開始
            let t0 = std::time::Instant::now();

            // Synthetic input
            let mut imu_msg = input.clone();

            // Apply offsets
            imu_msg.angular_velocity.x -= offset_x;
            imu_msg.angular_velocity.y -= offset_y;
            imu_msg.angular_velocity.z -= offset_z;

            // Covariances
            imu_msg.angular_velocity_covariance = vec![0.0; 9];
            imu_msg.angular_velocity_covariance[0] = stddev_xx * stddev_xx;
            imu_msg.angular_velocity_covariance[4] = stddev_yy * stddev_yy;
            imu_msg.angular_velocity_covariance[8] = stddev_zz * stddev_zz;
            imu_msg.linear_acceleration_covariance = vec![0.0; 9];
            imu_msg.linear_acceleration_covariance[0] = accel_stddev * accel_stddev;
            imu_msg.linear_acceleration_covariance[4] = accel_stddev * accel_stddev;
            imu_msg.linear_acceleration_covariance[8] = accel_stddev * accel_stddev;

            let mut out_msg = r2r::sensor_msgs::msg::Imu::default();
            // Transform
            out_msg.header.stamp = imu_msg.header.stamp.clone();
            out_msg.header.frame_id = output_frame_cloned.clone();
            out_msg.linear_acceleration = rotate_vector_by_quat(&imu_msg.linear_acceleration, q);
            out_msg.linear_acceleration_covariance = transform_covariance(&imu_msg.linear_acceleration_covariance);
            out_msg.angular_velocity = rotate_vector_by_quat(&imu_msg.angular_velocity, q);
            out_msg.angular_velocity_covariance = transform_covariance(&imu_msg.angular_velocity_covariance);

            // 計測終了
            let dt = t0.elapsed();
            if iter >= warmup_iterations_cl {
                let raw = dt.as_nanos();
                let corrected = raw.saturating_sub(overhead_per_call_ns_cl as u128);
                if corrected < min_ns { min_ns = corrected; }
                if corrected > max_ns_v { max_ns_v = corrected; }
                n += 1;
                let x = corrected as f64;
                let delta = x - mean_ns;
                mean_ns += delta / n as f64;
                let delta2 = x - mean_ns;
                m2 += delta * delta2;

                // Reservoir
                if (seen as usize) < RESERVOIR_K {
                    reservoir.push(corrected);
                } else {
                    let r = lcg_next(&mut rng_state);
                    let j = (r % (seen + 1)) as usize;
                    if j < RESERVOIR_K { reservoir[j] = corrected; }
                }
                seen += 1;
            }
        }

        // 集計と1回だけPublish
        if n > 1 {
            let variance = m2 / (n as f64 - 1.0);
            let stddev = variance.sqrt();
            let ci95 = 1.96_f64 * stddev / (n as f64).sqrt();
            let cv_percent = if mean_ns > 0.0 { (stddev / mean_ns) * 100.0 } else { f64::NAN };
            let (p50, p95, p99) = if reservoir.is_empty() {
                (f64::NAN, f64::NAN, f64::NAN)
            } else {
                let mut s = reservoir.clone();
                s.sort_unstable();
                let idx50 = s.len() / 2;
                let idx95 = (((s.len() as f64) * 0.95).ceil() as usize).saturating_sub(1).min(s.len()-1);
                let idx99 = (((s.len() as f64) * 0.99).ceil() as usize).saturating_sub(1).min(s.len()-1);
                (s[idx50] as f64, s[idx95] as f64, s[idx99] as f64)
            };
            println!(
                "[imu_corrector][bench] samples={} warmup={} mean={:.2} ns stddev={:.2} ns cv={:.2}% ci95=±{:.2} ns min={} ns p50={:.0} ns p95={:.0} ns p99={:.0} ns max={} ns",
                n, warmup_iterations_cl, mean_ns, stddev, cv_percent, ci95, min_ns, p50, p95, p99, max_ns_v
            );
        } else if n == 1 {
            println!(
                "[imu_corrector][bench] samples=1 warmup={} mean={:.2} ns min={} ns max={} ns stddev=NaN ns",
                warmup_iterations_cl, mean_ns, min_ns, max_ns_v
            );
        }

        if let Err(e) = pub_out.publish(&pub_msg) {
            eprintln!("imu_corrector publish error: {:?}", e);
        }
        done_for_task.store(true, Ordering::SeqCst);
    })?;

    // Spin
    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
        if done.load(Ordering::SeqCst) {
            break;
        }
    }
    Ok(())
}
