use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::{Duration, Instant, Sleep};

#[pin_project(project = SlowFuture1Proj)]
pub struct SlowFuture1<F> {
    #[pin]
    inner: F,
    #[pin]
    sleep: Sleep,
}

impl<F> Future for SlowFuture1<F>
where
    F: Future,
{
    type Output = F::Output;

    // ensures sleep is met before trying to get response
    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("slow1 polled");
        let this = self.project();
        let poll = this.sleep.poll(cx);
        match poll {
            Poll::Ready(_) => this.inner.poll(cx),
            Poll::Pending => {
                println!("slow1 sleep pending");
                Poll::Pending
            }
        }
    }
}

#[pin_project(project = SlowFuture2Proj)]
pub struct SlowFuture2<F: Future> {
    #[pin]
    inner: F,
    #[pin]
    sleep: Sleep,
    output: Option<F::Output>,
}

impl<F> Future for SlowFuture2<F>
where
    F: Future,
    F::Output: std::fmt::Debug,
{
    type Output = F::Output;

    // ensures response comes first before waiting
    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("slow2 polled");
        let this = self.project();
        if this.output.is_none() {
            let Poll::Ready(a) = this.inner.poll(cx) else {
                println!("slow2 inner pending");
                return Poll::Pending;
            };
            dbg!(&a);
            *this.output = Some(a);
            println!("assigned output");
        };
        let poll = this.sleep.poll(cx);
        match poll {
            Poll::Ready(_) => Poll::Ready(this.output.take().expect("set output to some earlier")),
            Poll::Pending => {
                println!("slow2 sleep pending");
                Poll::Pending
            }
        }
    }
}

pub struct SlowFuture3<F> {
    inner: F,
    sleep: Sleep,
}

impl<F: Future> SlowFuture3<F> {
    async fn run(self) -> F::Output {
        self.sleep.await;
        self.inner.await
    }
}

#[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() -> Result<(), tokio::io::Error> {
    {
        let start = Instant::now();
        let mut f = File::open("/dev/urandom").await?;
        let mut buf = vec![0u8; 8];
        let read_fut = f.read_exact(&mut buf);
        let slow_f = SlowFuture1 {
            inner: read_fut,
            // note to reader: this immediately sets the deadline
            sleep: tokio::time::sleep(Duration::from_secs(3)),
        };
        tokio::time::sleep(Duration::from_secs(2)).await;
        let res = slow_f.await?;
        dbg!(&buf, res);
        // runs for 3 seconds, not 5, since deadline was set above
        dbg!(Instant::now() - start);
    }
    {
        let start = Instant::now();
        let mut f = File::open("/dev/zero").await?;
        let mut buf = vec![88u8; 8];
        let read_fut = f.read_exact(&mut buf);
        let slow_f = SlowFuture2 {
            inner: read_fut,
            // note to reader: this immediately sets the deadline
            sleep: tokio::time::sleep(Duration::from_secs(3)),
            output: None,
        };
        let res = slow_f.await?;
        dbg!(&buf, res);
        // runs for 3 seconds, not 5, since deadline was set above
        dbg!(Instant::now() - start);
    }
    {
        let start = Instant::now();
        let mut f = File::open("/dev/zero").await?;
        let mut buf = vec![88u8; 8];
        let read_fut = f.read_exact(&mut buf);
        let slow_f = SlowFuture3 {
            inner: read_fut,
            // note to reader: this immediately sets the deadline
            sleep: tokio::time::sleep(Duration::from_secs(3)),
        };
        let res = slow_f.run().await?;
        dbg!(&buf, res);
        // runs for 3 seconds, not 5, since deadline was set above
        dbg!(Instant::now() - start);
    }

    {
        let start = Instant::now();
        let mut f = File::open("/dev/zero").await?;
        let mut buf = vec![88u8; 8];
        let read_fut = f.read_exact(&mut buf);
        let slow_f = async {
            tokio::time::sleep(Duration::from_secs(3)).await;
            read_fut.await
        };
        tokio::time::sleep(Duration::from_secs(2)).await;
        let res = slow_f.await?;
        dbg!(&buf, res);
        // this will for 5 seconds
        dbg!(Instant::now() - start);
    }
    Ok(())
}
