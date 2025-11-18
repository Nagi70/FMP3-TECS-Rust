use crate::tecs_global::*;
use crate::tecs_celltype::t_vehicle_velocity_converter::*;
use crate::tecs_signature::s_task_body::*;

struct VelocityReport {
    header: r2r::std_msgs::msg::Header,
    longitudinal_velocity: f64,
    lateral_velocity: f64,
    heading_rate: f64,
}

impl STaskBody for ETaskbodyForTVehicleVelocityConverter{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

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
				frame_id: "base_link".to_string(),
			},
			longitudinal_velocity: 3.0,
			lateral_velocity: 0.1,
			heading_rate: 0.2,
		};

		// ===================== 計測セットアップ =====================
		const WARMUP_ITERS: usize = 10_000;        // ウォームアップ回数
		const MEASURE_ITERS: usize = 2_000_000; // 計測対象回数（200万）

		// elapsed() 呼び出しオーバーヘッドの概算
		const OVERHEAD_SAMPLES: usize = 2_000_000;
		let overhead_reference = std::time::Instant::now();
		let overhead_start = std::time::Instant::now();
		for _ in 0..OVERHEAD_SAMPLES { let _ = overhead_reference.elapsed(); }
		let overhead_total = overhead_start.elapsed();
		let overhead_per_call_ns = overhead_total.as_nanos() as f64 / OVERHEAD_SAMPLES as f64;
		println!("[vehicle_velocity_converter][bench] elapsed() overhead ≈ {:.2} ns ({} samples)", overhead_per_call_ns, OVERHEAD_SAMPLES);

		// 統計用（ストリーミング集計: Welford）
		let mut n: u64 = 0;
		let mut mean_ns: f64 = 0.0;
		let mut m2: f64 = 0.0;
		let mut min_ns: u128 = u128::MAX;
		let mut max_ns: u128 = 0;

		// 近似分位点用 Reservoir Sampling
		const RESERVOIR_K: usize = 65_536;
		let mut reservoir: Vec<u128> = Vec::with_capacity(RESERVOIR_K);
		let mut seen: u64 = 0;
		// 簡易LCG RNG
		let mut rng_state: u64 = {
			let t = std::time::Instant::now().elapsed().as_nanos() as u64;
			0xD1B54A32D192ED03u64 ^ t
		};
		let mut lcg_next = |state: &mut u64| -> u64 {
			*state = state.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
			*state
		};


		// ===================== ウォームアップ + 計測ループ =====================
		for iter in 0..(WARMUP_ITERS + MEASURE_ITERS) {
			let t0 = std::time::Instant::now();

			if velocity_report.header.frame_id != *lg.frame_id {
				continue;
			}

			let mut twist_msg = r2r::geometry_msgs::msg::TwistWithCovarianceStamped::default();

			// Build TwistWithCovarianceStamped message
			twist_msg.header = velocity_report.header.clone();
			twist_msg.twist.twist.linear.x = velocity_report.longitudinal_velocity * *lg.speed_scale_factor;
			twist_msg.twist.twist.linear.y = velocity_report.lateral_velocity;
			twist_msg.twist.twist.angular.z = velocity_report.heading_rate;

			// Set covariances (diagonal elements)
			twist_msg.twist.covariance = vec![0.0; 36];
			twist_msg.twist.covariance[0 + 0 * 6] = *lg.velocity_stddev_xx * *lg.velocity_stddev_xx; // vx variance
			twist_msg.twist.covariance[1 + 1 * 6] = 10000.0;
			twist_msg.twist.covariance[2 + 2 * 6] = 10000.0;
			twist_msg.twist.covariance[3 + 3 * 6] = 10000.0;
			twist_msg.twist.covariance[4 + 4 * 6] = 10000.0;
			twist_msg.twist.covariance[5 + 5 * 6] = *lg.angular_velocity_stddev_zz * *lg.angular_velocity_stddev_zz; // wz variance
			
			// 計測終了
			let dt = t0.elapsed();
			if iter >= WARMUP_ITERS {
				let raw = dt.as_nanos();
				let corrected = raw.saturating_sub(overhead_per_call_ns as u128);
				if corrected < min_ns { min_ns = corrected; }
				if corrected > max_ns { max_ns = corrected; }
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

			// OPTIONAL: ウォームアップ進捗表示（抑制）
			// if iter + 1 == WARMUP_ITERS { println!("[vehicle_velocity_converter][bench] warmup done"); }
		}

		// ===================== 集計 =====================
		if n > 1 {
			let variance = m2 / (n as f64 - 1.0);
			let stddev = variance.sqrt();
			let ci95 = 1.96_f64 * stddev / (n as f64).sqrt();
			let cv_percent = if mean_ns > 0.0 { (stddev / mean_ns) * 100.0 } else { f64::NAN };
			let (p50, p95, p99) = if reservoir.is_empty() { (f64::NAN, f64::NAN, f64::NAN) } else {
				let mut s = reservoir.clone();
				s.sort_unstable();
				let idx50 = s.len()/2;
				let idx95 = (((s.len() as f64)*0.95).ceil() as usize).saturating_sub(1).min(s.len()-1);
				let idx99 = (((s.len() as f64)*0.99).ceil() as usize).saturating_sub(1).min(s.len()-1);
				(s[idx50] as f64, s[idx95] as f64, s[idx99] as f64)
			};
			println!(
				"[vehicle_velocity_converter][bench] samples={} warmup={} mean={:.2} ns stddev={:.2} ns cv={:.2}% ci95=±{:.2} ns min={} ns p50={:.0} ns p95={:.0} ns p99={:.0} ns max={} ns",
				n, WARMUP_ITERS, mean_ns, stddev, cv_percent, ci95, min_ns, p50, p95, p99, max_ns
			);
		} else if n == 1 {
			println!(
				"[vehicle_velocity_converter][bench] samples=1 warmup={} mean={:.2} ns min={} ns max={} ns stddev=NaN ns",
				WARMUP_ITERS, mean_ns, min_ns, max_ns
			);
		}
	}
}

