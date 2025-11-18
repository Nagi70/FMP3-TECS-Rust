# ROS 2 + Rust (R2R) nodes

Small example nodes using the [r2r](https://github.com/sequenceplanner/r2r) crate:
- `talker`: publishes `std_msgs/msg/String` to `/chatter` at 2 Hz
- `listener`: subscribes to `/chatter` and prints messages
- `imu_corrector`: subscribes `sensor_msgs/Imu` on `input`, applies a dummy TF rotation + covariance handling, and publishes corrected IMU on `output`
- `gyro_odometer`: publishes synthetic `geometry_msgs/TwistStamped` and `geometry_msgs/TwistWithCovarianceStamped`
- `vehicle_velocity_converter`: uses a wall timer and fixed values (parameters) to publish `geometry_msgs/TwistWithCovarianceStamped` on `twist_with_covariance`

## Prerequisites
- Linux
- ROS 2 (Humble, Iron, or Jazzy) installed and sourced
- libclang (Ubuntu: `sudo apt-get install -y libclang-dev`)
- Rust toolchain (`rustup` + `cargo`)

## Build and run
1) Open a new shell and source your ROS 2 installation (adjust distro):

```bash
source /opt/ros/humble/setup.sh
```

2) Build:

```bash
cargo build
```

3) Run the talker:

```bash
cargo run --bin talker
```

4) In another shell (also sourced), run the listener:

```bash
cargo run --bin listener
```

You should see the listener print lines like `I heard: Hello from r2r: 0`.

## Faster builds (message filter)
This project uses `IDL_PACKAGE_FILTER` in `.cargo/config.toml` to limit which message
packages are code-generated. It's currently set to:

```
IDL_PACKAGE_FILTER = "std_msgs;builtin_interfaces;geometry_msgs;sensor_msgs;autoware_vehicle_msgs"
```

Extend or adjust this list as needed.

## Notes
- If you change sourced workspaces or message definitions, run:

```bash
cargo clean -p r2r_msg_gen
```

so the message bindings regenerate on next build.

### Converter behavior
The current `vehicle_velocity_converter` does not depend on Autoware message packages. It publishes
fixed values at 20 Hz using a wall timer. You can adjust the values with these parameters:

- `frame_id` (string, default: `base_link`)
- `velocity_stddev_xx` (float, default: `0.0`)
- `angular_velocity_stddev_zz` (float, default: `0.0`)
- `speed_scale_factor` (float, default: `1.0`)
- `fixed_longitudinal_velocity` (float, default: `3.0`)
- `fixed_lateral_velocity` (float, default: `0.1`)
- `fixed_heading_rate` (float, default: `0.2`)
