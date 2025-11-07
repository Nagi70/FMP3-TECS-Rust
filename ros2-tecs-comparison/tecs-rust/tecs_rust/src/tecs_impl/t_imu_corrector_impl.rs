use crate::tecs_global::*;
use crate::tecs_celltype::t_imu_corrector::*;
use crate::tecs_signature::{s_task_body::*, s_tf::*};
impl STaskBody for ETaskbodyForTImuCorrector{

	fn main(&self) {
		let lg = self.cell.get_cell_ref();

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

		// ===================== 計測セットアップ =====================
		const WARMUP_ITERS: usize = 10_000;        // ウォームアップ回数（統計に含めない）
		const MEASURE_ITERS: usize = 2_000_000; // 計測対象回数（200万）

		// elapsed() 呼び出しオーバーヘッドの概算（サンプル数は大きめにしてノイズ低減）
		let overhead_reference = std::time::Instant::now();
		let overhead_start = std::time::Instant::now();
		const OVERHEAD_SAMPLES: usize = 2_000_000;
		for _ in 0..OVERHEAD_SAMPLES { let _ = overhead_reference.elapsed(); }
		let overhead_total = overhead_start.elapsed();
		let overhead_per_call_ns = overhead_total.as_nanos() as f64 / OVERHEAD_SAMPLES as f64;
		println!("[imu_corrector][bench] elapsed() overhead ≈ {:.2} ns ({} samples)", overhead_per_call_ns, OVERHEAD_SAMPLES);

		// 統計用（ストリーミング集計: Welford）
		let mut n: u64 = 0;
		let mut mean_ns: f64 = 0.0;
		let mut m2: f64 = 0.0;
		let mut min_ns: u128 = u128::MAX;
		let mut max_ns: u128 = 0;

		// 近似分位点用のリザーバサンプリング（メモリ節約）
		const RESERVOIR_K: usize = 65_536; // 約65kサンプル（約1MB程度）
		let mut reservoir: Vec<u128> = Vec::with_capacity(RESERVOIR_K);
		let mut seen: u64 = 0;
		// 簡易LCG RNG（依存追加なし）
		let mut rng_state: u64 = {
			let t = std::time::Instant::now().elapsed().as_nanos() as u64;
			// 非常に簡易なシード
			0x9E3779B97F4A7C15u64 ^ t
		};
		let mut lcg_next = |state: &mut u64| -> u64 {
			*state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
			*state
		};

		// ===================== ウォームアップ + 計測ループ =====================
		for iter in 0..(WARMUP_ITERS + MEASURE_ITERS) {
			let t0 = std::time::Instant::now();

            // Build a synthetic incoming Imu message from fixed values
            let mut imu_msg = input.clone();

            // Apply offsets
            imu_msg.angular_velocity.x -= *lg.angular_velocity_offset_x;
            imu_msg.angular_velocity.y -= *lg.angular_velocity_offset_y;
            imu_msg.angular_velocity.z -= *lg.angular_velocity_offset_z;

            // Set covariances (diagonal elements)
            imu_msg.angular_velocity_covariance = vec![0.0; 9];
            imu_msg.angular_velocity_covariance[0] = *lg.angular_velocity_stddev_xx * *lg.angular_velocity_stddev_xx;
            imu_msg.angular_velocity_covariance[4] = *lg.angular_velocity_stddev_yy * *lg.angular_velocity_stddev_yy;
            imu_msg.angular_velocity_covariance[8] = *lg.angular_velocity_stddev_zz * *lg.angular_velocity_stddev_zz;
            imu_msg.linear_acceleration_covariance = vec![0.0; 9];
            imu_msg.linear_acceleration_covariance[0] = *lg.accel_stddev * *lg.accel_stddev;
            imu_msg.linear_acceleration_covariance[4] = *lg.accel_stddev * *lg.accel_stddev;
            imu_msg.linear_acceleration_covariance[8] = *lg.accel_stddev * *lg.accel_stddev;

			let mut out_msg = r2r::sensor_msgs::msg::Imu::default();

            out_msg.header.stamp = imu_msg.header.stamp.clone();
            out_msg.header.frame_id = lg.output_frame.to_string();
            out_msg.linear_acceleration = lg.c_tf.rotate_vector_by_quat(&imu_msg.linear_acceleration);
            out_msg.linear_acceleration_covariance = lg.c_tf.transform_covariance(&imu_msg.linear_acceleration_covariance);
            out_msg.angular_velocity = lg.c_tf.rotate_vector_by_quat(&imu_msg.angular_velocity);
            out_msg.angular_velocity_covariance = lg.c_tf.transform_covariance(&imu_msg.angular_velocity_covariance);

			// 計測終了
			let dt = t0.elapsed();
			if iter >= WARMUP_ITERS { // 計測対象
				// オーバーヘッド補正（負にならないように0クリップ）
				let raw = dt.as_nanos();
				let corrected = raw.saturating_sub(overhead_per_call_ns as u128);
				if corrected < min_ns { min_ns = corrected; }
				if corrected > max_ns { max_ns = corrected; }
				// Welford: 更新
				n += 1;
				let x = corrected as f64;
				let delta = x - mean_ns;
				mean_ns += delta / n as f64;
				let delta2 = x - mean_ns;
				m2 += delta * delta2;

				// Reservoir sampling
				if (seen as usize) < RESERVOIR_K {
					reservoir.push(corrected);
				} else {
					let r = lcg_next(&mut rng_state);
					let j = (r % (seen + 1)) as usize; // [0, seen]
					if j < RESERVOIR_K { reservoir[j] = corrected; }
				}
				seen += 1;
			}

			// OPTIONAL: ウォームアップ進捗表示（抑制）
			// if iter + 1 == WARMUP_ITERS { println!("[imu_corrector][bench] warmup done"); }
		}

		// ===================== 集計 =====================
		if n > 1 {
			let variance = m2 / (n as f64 - 1.0);
			let stddev = variance.sqrt();
			// 95%信頼区間（正規近似）
			let ci95 = 1.96_f64 * stddev / (n as f64).sqrt();
			// 変動係数
			let cv_percent = if mean_ns > 0.0 { (stddev / mean_ns) * 100.0 } else { f64::NAN };

			// リザーバから分位点推定
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
				n, WARMUP_ITERS, mean_ns, stddev, cv_percent, ci95, min_ns, p50, p95, p99, max_ns
			);
		} else if n == 1 {
			println!(
				"[imu_corrector][bench] samples=1 warmup={} mean={:.2} ns min={} ns max={} ns stddev=NaN ns",
				WARMUP_ITERS, mean_ns, min_ns, max_ns
			);
		}
	}
}

