use std::time::Duration;

use tokio::time::Instant;

#[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() {
    {
        let start = Instant::now();
        // this block finishes in 2 seconds instead of 3. pretty interesting
        let s1 = tokio::time::sleep(Duration::from_secs(1));
        tokio::time::sleep(Duration::from_secs(2)).await;
        s1.await;
        dbg!(Instant::now() - start);
    }
    {
        let start = Instant::now();
        // this block will finish in 3 seconds
        let s1 = async { tokio::time::sleep(Duration::from_secs(1)).await };
        tokio::time::sleep(Duration::from_secs(2)).await;
        s1.await;
        dbg!(Instant::now() - start);
    }
}
