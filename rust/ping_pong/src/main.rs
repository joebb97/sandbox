use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[allow(dead_code)]
fn main_channels() {
    let (ping_tx, ping_rx): (Sender<&str>, Receiver<&str>) = mpsc::channel();
    let (pong_tx, pong_rx): (Sender<&str>, Receiver<&str>) = mpsc::channel();
    let num_times = 5;

    let ping_thread = thread::spawn(move || {
        for _ in 0..num_times {
            println!("{:?}", ping_rx.recv().unwrap());
            pong_tx.send("pong").unwrap();
        }
    });
    let pong_thread = thread::spawn(move || {
        for _ in 0..num_times {
            ping_tx.send("ping").unwrap();
            println!("{:?}", pong_rx.recv().unwrap());
        }
    });
    ping_thread.join().unwrap();
    pong_thread.join().unwrap();

}

fn main_mutex() {
    let num_times = 5;
    // let pair = Arc::new((Mutex::new(Some("ping")), Condvar::new()));
    let pair = Arc::new((Mutex::new("ping"), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    let ping_thread = thread::spawn(move || {
        for _ in 0..num_times {
            let &(ref lock, ref cv) = &*pair2;
            let mut print_str = lock.lock().unwrap();
            while *print_str == "pong" {
                print_str = cv.wait(print_str).unwrap();

            }
            println!("{:?}", *print_str);
            *print_str = "pong";
            cv.notify_one();
        }
    });
    let pong_thread = thread::spawn(move || {
        for _ in 0..num_times {
            let &(ref lock, ref cv) = &*Arc::clone(&pair);
            let mut print_str = lock.lock().unwrap();
            while *print_str == "ping" {
                print_str = cv.wait(print_str).unwrap();

            }
            println!("{:?}", *print_str);
            *print_str = "ping";
            cv.notify_one();
        }
    });
    ping_thread.join().unwrap();
    pong_thread.join().unwrap();
}
fn main() {
    // main_channels();
    main_mutex();
}
