use std::{thread, time};

async fn say_world() {
    println!("world");
    thread::sleep(time::Duration::from_secs(2));
}

#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();
    tokio::spawn(async {
        op.await;
    });

    // Uncomment this line and the order should flip. It's undefined though
    // since this is a race condition
    thread::sleep(time::Duration::from_millis(100));
    // This println! comes first
    println!("hello");
}
