use futures::future;
use tokio::time::{sleep, Duration};

async fn say_world(num_secs: u64) {
    sleep(Duration::from_secs(num_secs)).await;
    println!("world");
}

#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let _ = future::join_all(vec![
        say_world(1),
        say_world(2),
        say_world(1),
    ]).await;

    // Uncomment this line and the order should flip. It's undefined though
    // since this is a race condition
    // This println! comes first
    println!("hello");
}
