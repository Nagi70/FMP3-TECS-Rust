use futures::{executor::LocalPool, task::LocalSpawnExt};
use r2r::QosProfile;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ROS 2 context and create a node
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "r2r_talker", "")?;

    // Create a publisher on /chatter
    let p = node.create_publisher::<r2r::std_msgs::msg::String>("/chatter", QosProfile::default())?;

    // Create a wall timer to tick at 2 Hz
    let mut timer = node.create_wall_timer(Duration::from_millis(500))?;

    // Local single-threaded executor
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    // Publish on each timer tick
    spawner.spawn_local(async move {
        let mut count: u64 = 0;
        loop {
            // tick returns Result; ignore errors but keep loop alive
            let _ = timer.tick().await;
            let msg = r2r::std_msgs::msg::String { data: format!("Hello from r2r: {}", count) };
            if let Err(e) = p.publish(&msg) {
                eprintln!("publish error: {:?}", e);
            }
            count += 1;
        }
    })?;

    // Spin the node and poll the task executor
    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
