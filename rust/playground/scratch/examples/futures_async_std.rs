use async_std::io::{self, Write as _};
use async_std::sync::Mutex;
use core::time;
use futures::channel::mpsc;
use futures::{executor, AsyncWriteExt, SinkExt};
use futures::{executor::ThreadPool, StreamExt};
use futures_time::task::sleep;
use futures_time::time::Duration;
use http::header::*;
use http::uri::Scheme;
use http::Request;
use std::fmt::Write as _;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

fn main() {
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (mut tx, mut rx) = mpsc::channel::<String>(100);

    // Create a future by an async block, where async is responsible for an
    // implementation of Future. At this point no executor has been provided
    // to this future, so it will not be running.
    let fut_values = async {
        let receiver = async move {
            loop {
                match rx.next().await {
                    None => panic!("I've been closed!!!!!"),
                    Some(x) => {
                        println!("got {x}");
                        futures_time::task::sleep(Duration::from_secs(1)).await;
                    }
                }
                println!("outer loop");
            }
        };

        let mut sender_tx = tx.clone();
        let sender = async move {
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            loop {
                stdout.write_all(b"> ").await.unwrap();
                stdout.flush().await.unwrap();
                let mut line = String::new();
                stdin.read_line(&mut line).await.unwrap();
                let s = line.trim();
                if s.is_empty() {
                    continue;
                }
                sender_tx.try_send(s.to_string()).unwrap();
            }
        };
        let tx = Arc::new(Mutex::new(tx));
        let mut sent = 0;
        let sender1_tx = tx.clone();
        let sender1 = async move {
            loop {
                // sender1_tx.lock().await.try_send(format!("hi {sent} from sender1")).unwrap();
                sender1_tx
                    .lock()
                    .await
                    .send(format!("hi {sent} from sender1"))
                    .await
                    .unwrap();
                sent += 1;
                println!("sender1 {sent}");
                sleep(Duration::from_millis(100)).await;
            }
        };

        //         let mut sent = 0;
        //         let sender2 = async move {
        //             loop {
        //                 tx.lock().await.send("hi {sent} from sender2".to_string()).await.unwrap();
        //                 sent += 1;
        //                 println!("sender2 {sent}");
        //                 sleep(Duration::from_millis(50)).await;
        //             }
        //         };

        // let other = async {
        //     loop {
        //         let time = SystemTime::now();
        //         println!("{time:?}");
        //         futures_time::task::sleep(Duration::from_secs(1)).await;
        //     }
        // };

        futures::join!(receiver, sender)
    };

    executor::block_on(fut_values);
}

fn _main() {
    let req: Request<Option<&[u8]>> = Request::builder()
        .method(http::Method::GET)
        .uri("https://hey.com/path/thing")
        .header("HOST", "example.com")
        .body(None)
        .unwrap();
    if req.uri() == "hey.com" {
        println!("HEY")
    }
    // let (mut parts, body) = req.into_parts();
    // let mut uri = parts.uri.into_parts();
    // uri.scheme = Some(Scheme::HTTPS);
    // let uri = http::Uri::from_parts(uri).unwrap();
    // parts.uri = uri;
    // let new = Request::from_parts(parts, body);
    // dbg!(&new);
}

fn main2() {
    let path = "/tmp/tmp.socket";
    let listener = UnixListener::bind(path).unwrap();
    thread::spawn(move || {
        let socket = match UnixStream::connect(path) {
            Ok(sock) => {
                println!("Connected to sock");
                sock
            }
            Err(e) => {
                println!("Couldn't connect: {e:?}");
                return;
            }
        };
    });
    let _ = listener.accept();
    std::thread::sleep(time::Duration::from_secs(1));
    let _ = std::fs::remove_file(path);
}

struct Thing {}

impl Drop for Thing {
    fn drop(&mut self) {
        println!("oops")
    }
}

fn main22() {
    let host = "https://123.245.22.33:8080/thing";
    // let b = url::Url::parse(host).unwrap();
    // println!("{}", b.host_str().unwrap());

    let a = Request::builder()
        .method(http::Method::GET)
        .uri(host)
        .header("Host", "hello,goodbye")
        .header("host", "oops")
        .header("gloop", "poop")
        .body(())
        .unwrap();
    println!("{:?}", a.uri().host());
    for (k, v) in a.headers().iter() {
        println!("{}: {}", k.as_str(), v.to_str().unwrap());
    }

    let mut t = Thing {};
    for i in (0..10) {
        t = Thing {};
    }

    // let (parts, _) = a.into_parts();
    // let mut preface = Vec::new();
    // write!(preface, "{} {}", parts.method.as_str(), parts.uri.path()).unwrap();
    // println!("{:?}", String::from_utf8(preface).unwrap());
}

fn main11() {
    let mut map = HeaderMap::new();

    map.insert(HOST, "hello,goodbye".parse().unwrap());
    map.append("host", "goodbye".parse().unwrap());
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());

    println!("aggggg {:?}", map.get("host"));

    println!("{}", make_headers(map).unwrap());
}

fn make_headers(headers: http::HeaderMap) -> Result<String, Box<dyn std::error::Error>> {
    let mut ret = String::new();
    for key in headers.keys() {
        let mut all_vals = Vec::new();
        for val in headers.get_all(key) {
            all_vals.push(val.to_str()?)
        }
        let all_vals = all_vals.join(",");
        write!(ret, "{key}: {all_vals}\r\n")?;
    }
    Ok(ret)
}
