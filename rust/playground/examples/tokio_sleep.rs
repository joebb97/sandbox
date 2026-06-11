use futures::future;
use tokio::time::{sleep, Duration};

async fn say_world(num_secs: u64) {
    sleep(Duration::from_secs(num_secs)).await;
    println!("world");
}

fn thing() {
    let b = 88;
    tokio::spawn(async move {
        let a = 3;
        println!("running from non-async {a} {b}");
    });
}

#[tokio::main]
async fn main() {
    let _ = future::join_all(vec![say_world(1), say_world(2), say_world(1)]).await;
    thing();
    sleep(Duration::from_secs(5)).await;
}
