use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{self, AsyncRead, AsyncWriteExt, ReadBuf},
    net::TcpListener,
};

struct LoggingReader<R> {
    inner: R,
    peer: String,
}

impl<R> LoggingReader<R> {
    fn new(inner: R, peer: impl Into<String>) -> Self {
        Self {
            inner,
            peer: peer.into(),
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for LoggingReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let before = buf.filled().len();
        let result = Pin::new(&mut self.inner).poll_read(cx, buf);

        if let Poll::Ready(Ok(())) = &result {
            let after = buf.filled().len();

            if after > before {
                let bytes = &buf.filled()[before..after];
                println!(
                    "received from {}: {:?}",
                    self.peer,
                    String::from_utf8_lossy(bytes)
                );
            }
        }

        result
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("echo server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("client connected: {addr}");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = LoggingReader::new(reader, addr.to_string());

            if let Err(err) = tokio::io::copy(&mut reader, &mut writer).await {
                eprintln!("connection error for {addr}: {err}");
            }

            if let Err(err) = writer.shutdown().await {
                eprintln!("shutdown error for {addr}: {err}");
            }

            println!("client disconnected: {addr}");
        });
    }
}
