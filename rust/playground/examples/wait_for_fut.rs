use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, ReadBuf};

struct WaitForBytes<R: AsyncRead + Unpin> {
    reader: R,
    to_read: usize,
    read: usize,
    buf: Vec<u8>,
}

impl<R: AsyncRead + Unpin> WaitForBytes<R> {
    fn new(reader: R, to_read: usize, buf_size: usize) -> Self {
        Self {
            reader,
            to_read,
            read: 0,
            buf: vec![0; buf_size],
        }
    }
}

impl<R: AsyncRead + Unpin> Future for WaitForBytes<R> {
    type Output = io::Result<Vec<u8>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("poll called WaitForBytes");
        loop {
            let this = &mut *self;
            let mut read_buf = ReadBuf::new(&mut this.buf[this.read..]);
            let poll = Pin::new(&mut this.reader).poll_read(cx, &mut read_buf);
            match poll {
                Poll::Ready(Ok(_)) => {
                    let n = read_buf.filled().len();
                    println!("read some bytes {n:?}");

                    if n == 0 {
                        let out = std::mem::take(&mut this.buf);
                        return Poll::Ready(Ok(out)); // EOF
                    }

                    this.read += n;

                    if this.read >= this.to_read {
                        let out = std::mem::take(&mut this.buf);
                        return Poll::Ready(Ok(out));
                    }
                }
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => {
                    println!("we're pending");
                    return Poll::Pending;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let w = WaitForBytes::new(stdin, 5, 1024);
    let buf = w.await.unwrap();
    dbg!(&buf[0..7]);
}
