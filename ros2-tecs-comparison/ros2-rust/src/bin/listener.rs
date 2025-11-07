use futures::{executor::LocalPool, stream::StreamExt, task::LocalSpawnExt};
use r2r::QosProfile;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ROS 2 context and create a node
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "r2r_listener", "")?;

    // Subscribe to /chatter
    let mut sub = node.subscribe::<r2r::std_msgs::msg::String>("/chatter", QosProfile::default())?;

    // Local single-threaded executor
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    // Print every received message
    spawner.spawn_local(async move {
        while let Some(msg) = sub.next().await {
            println!("I heard: {}", msg.data);
        }
    })?;

    // Spin the node and poll the task executor
    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
