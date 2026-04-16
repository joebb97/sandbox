use std::time::Duration;

use tokio::time::sleep;
use tokio_util::sync::CancellationToken; // 0.7.18

#[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() {
    let ct = CancellationToken::new();
    let ctc = ct.child_token();
    let dg = ctc.clone().drop_guard();
    let jh = tokio::spawn(
        ctc.run_until_cancelled_owned(async move { sleep(Duration::from_secs(30)).await }),
    );
    tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;
        ct.cancel();
    });
    let a = jh.await;
    println!("{a:?}")
}
