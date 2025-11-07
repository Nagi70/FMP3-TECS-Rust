use futures::{executor::LocalPool, task::LocalSpawnExt};
use r2r::QosProfile;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

// Timer-based converter: use fixed values instead of subscribing to VelocityReport

struct VelocityReport {
    header: r2r::std_msgs::msg::Header,
    longitudinal_velocity: f64,
    lateral_velocity: f64,
    heading_rate: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "vehicle_velocity_converter", "")?;

    // Parameters
    let frame_id: String = node
        .get_parameter::<String>("frame_id")
        .unwrap_or_else(|_| "base_link".to_string());
    let stddev_vx: f64 = node.get_parameter::<f64>("velocity_stddev_xx").unwrap_or(0.2);
    let stddev_wz: f64 = node
        .get_parameter::<f64>("angular_velocity_stddev_zz")
        .unwrap_or(0.1);
    let speed_scale_factor: f64 = node
        .get_parameter::<f64>("speed_scale_factor")
        .unwrap_or(1.0);

    // Fixed values (simulate VelocityReport)
    let velocity_report = VelocityReport {
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
            frame_id: frame_id.clone(),
        },
        longitudinal_velocity: 3.0,
        lateral_velocity: 0.1,
        heading_rate: 0.2,
    };

    // Publisher
    let twist_cov_pub = node.create_publisher::<r2r::geometry_msgs::msg::TwistWithCovarianceStamped>(
        "twist_with_covariance",
        QosProfile::default(),
    )?;

    // Timer at 20 Hz
    let mut timer = node.create_wall_timer(std::time::Duration::from_millis(50))?;

    // Optional parameter to stop after N iterations (default: 1000)
    let max_iter_param: f64 = node.get_parameter::<f64>("max_iterations").unwrap_or(2_000_000.0);
    let max_iterations: u64 = if max_iter_param.is_finite() && max_iter_param > 0.0 {
        max_iter_param as u64
    } else { 1000 };
    let done = Arc::new(AtomicBool::new(false));

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    // ===================== 計測セットアップ =====================
    let warmup_param: f64 = node.get_parameter::<f64>("warmup_iterations").unwrap_or(10_000.0);
    let warmup_iterations: u64 = if warmup_param.is_finite() && warmup_param >= 0.0 { warmup_param as u64 } else { 10_000 };
    const OVERHEAD_SAMPLES: usize = 2_000_000;
    let overhead_reference = std::time::Instant::now();
    let overhead_start = std::time::Instant::now();
    for _ in 0..OVERHEAD_SAMPLES { let _ = overhead_reference.elapsed(); }
    let overhead_total = overhead_start.elapsed();
    let overhead_per_call_ns = overhead_total.as_nanos() as f64 / OVERHEAD_SAMPLES as f64;
    println!(
        "[vehicle_velocity_converter][bench] elapsed() overhead ≈ {:.2} ns ({} samples)",
        overhead_per_call_ns, OVERHEAD_SAMPLES
    );

    let done_for_task = done.clone();
    let overhead_per_call_ns_cl = overhead_per_call_ns;
    let warmup_iterations_cl = warmup_iterations;
    let max_iterations_cl = max_iterations;
    spawner.spawn_local(async move {
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
            0xB7E151628AED2A6Bu64 ^ t
        };
        let mut lcg_next = |state: &mut u64| -> u64 {
            *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            *state
        };

        // 最終的に1回だけPublishするメッセージ
        let mut pub_msg = r2r::geometry_msgs::msg::TwistWithCovarianceStamped::default();


        for iter in 0..(warmup_iterations_cl + max_iterations_cl) {
            // 計測開始
            let t0 = std::time::Instant::now();

            if velocity_report.header.frame_id != frame_id {
                continue;
            }
            
            let mut out = r2r::geometry_msgs::msg::TwistWithCovarianceStamped::default();

            // Build message
            out.header = velocity_report.header.clone();
            out.twist.twist.linear.x = velocity_report.longitudinal_velocity * speed_scale_factor;
            out.twist.twist.linear.y = velocity_report.lateral_velocity;
            out.twist.twist.angular.z = velocity_report.heading_rate;

            // 6x6 covariance (length 36)
            out.twist.covariance = vec![0.0; 36];
            out.twist.covariance[0 + 0 * 6] = stddev_vx * stddev_vx;
            out.twist.covariance[1 + 1 * 6] = 10000.0;
            out.twist.covariance[2 + 2 * 6] = 10000.0;
            out.twist.covariance[3 + 3 * 6] = 10000.0;
            out.twist.covariance[4 + 4 * 6] = 10000.0;
            out.twist.covariance[5 + 5 * 6] = stddev_wz * stddev_wz;

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
                "[vehicle_velocity_converter][bench] samples={} warmup={} mean={:.2} ns stddev={:.2} ns cv={:.2}% ci95=±{:.2} ns min={} ns p50={:.0} ns p95={:.0} ns p99={:.0} ns max={} ns",
                n, warmup_iterations_cl, mean_ns, stddev, cv_percent, ci95, min_ns, p50, p95, p99, max_ns_v
            );
        } else if n == 1 {
            println!(
                "[vehicle_velocity_converter][bench] samples=1 warmup={} mean={:.2} ns min={} ns max={} ns stddev=NaN ns",
                warmup_iterations_cl, mean_ns, min_ns, max_ns_v
            );
        }

        if let Err(e) = twist_cov_pub.publish(&pub_msg) {
            eprintln!("vehicle_velocity_converter publish error: {:?}", e);
        }
        done_for_task.store(true, Ordering::SeqCst);
    })?;

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
        pool.run_until_stalled();
        if done.load(Ordering::SeqCst) {
            break;
        }
    }
    Ok(())
}
