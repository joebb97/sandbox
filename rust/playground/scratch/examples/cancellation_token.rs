use std::time::Duration;

use tokio::time::sleep;
use tokio_util::future::FutureExt;
use tokio_util::sync::CancellationToken; // 0.7.18

#[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() {
    {
        // loops forever for some reason?
        // in any of the scenarios for jh, if this is just loop {} the whole example stops
        // let jh = tokio::spawn(async move { loop {} });
        let jh = tokio::spawn(async move { sleep(Duration::from_secs(30)).await });
        let ah = jh.abort_handle();
        tokio::spawn(async move {
            sleep(Duration::from_secs(3)).await;
            ah.abort();
        });
        let a = jh.await.unwrap_err();
        println!("{a:?}")
    };
    {
        let ct = CancellationToken::new();
        let jh = tokio::spawn(
            async move { sleep(Duration::from_secs(30)).await }
                .with_cancellation_token_owned(ct.clone()),
        );
        tokio::spawn(async move {
            sleep(Duration::from_secs(3)).await;
            ct.cancel();
        });
        let a = jh.await;
        println!("{a:?}")
    }
    {
        let ct = CancellationToken::new();
        let ctc = ct.clone();
        // let jh = tokio::spawn(ctc.run_until_cancelled_owned(async move { loop {} }));
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
}
