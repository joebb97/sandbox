use std::time::Duration;

use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() {
    {
        let (tx, mut rx) = mpsc::channel(8);
        let c = tx.clone();
        let jh1 = tokio::spawn(async move {
            let a = c;
            a.send("hey").await.unwrap();
        });
        let c = tx.clone();
        let jh2 = tokio::spawn(async move {
            let a = c;
            a.send("ho").await.unwrap();
            tokio::time::sleep(Duration::from_secs(8)).await;
            a.send("hhh").await.unwrap();
        });
        let jh3 = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(val) => {
                        println!("got {val}");
                        rx.close();
                    }
                    None => {
                        println!("got none, probably closed");
                        return;
                    }
                }
            }
        });
        drop(tx);
        jh1.await.expect("why panic");
        jh2.await.expect("why panic");
        jh3.await.expect("why panic");
        println!("done");
    }
    {
        println!("test 2");
        let (tx, mut rx) = mpsc::channel(8);
        let c = tx.clone();
        let jh1 = tokio::spawn(async move {
            let a = c;
            tokio::time::sleep(Duration::from_secs(8)).await;
            a.send("hey").await.unwrap();
        });
        let c = tx.clone();
        let jh2 = tokio::spawn(async move {
            let a = c;
            a.send("ho").await.unwrap();
        });
        let jh3 = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(val) => {
                        println!("got {val}");
                        break;
                    }
                    None => {
                        println!("got none, probably closed");
                        return;
                    }
                }
            }
        });
        drop(tx);
        jh1.await.expect("why panic");
        jh2.await.expect("why panic");
        jh3.await.expect("why panic");
        println!("done");
    }
}
