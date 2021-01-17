use clap::ArgMatches;
use std::thread;
use std::error::Error;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::str::from_utf8;

fn handle_connection(mut stream: TcpStream, buf_size: usize) -> Result<(), Box<dyn Error>> {
    let mut data = vec![0 as u8; buf_size]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            if size == 0 {
                false
            } else {
                println!("{}", from_utf8(&data[0..size])?);
                stream.write(&data[0..size])?;
                true
            }
        },
        Err(_) => {
            //println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
    Ok(())
}

pub fn run_server(app: ArgMatches) -> Result<(), Box<dyn Error>> {
    let addr: &str = app.value_of("address").ok_or("No address")?;
    let port: &str = app.value_of("port").ok_or("No port")?;
    let buf_size: usize = app.value_of("buffersize").ok_or("No buffsize")?.parse::<usize>()?;
    let combined = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(combined)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted connection {:?}", stream);
                thread::spawn(move || {
                    let _ = handle_connection(stream, buf_size);
                });
            }
            Err(e) => { 
                println!("an error");
                Err(e)?
            }
        }
    }
    // This won't be reached unless we handle a signal or set nonblocking.
    Ok(())
}

pub fn run_client(app: ArgMatches) -> Result<(), Box<dyn Error>> {
    let addr: &str = app.value_of("address").ok_or("No address")?;
    let port: &str = app.value_of("port").ok_or("No port")?;
    let combined = format!("{}:{}", addr, port);
    let stdin = io::stdin();
    match TcpStream::connect(combined) {
        Ok(mut stream) => {
            for line in stdin.lock().lines() {
                let line_ref: &String = &line?;
                let bytes = line_ref.as_bytes();
                stream.write(bytes)?;
                let mut data: Vec<u8> = vec![0 as u8; bytes.len()];
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        println!("{}", from_utf8(&data)?);
                    }
                    Err(e) => Err(e)?
                }
            }
        }
        Err(e) => Err(e)?
    }
    Ok(())
}

// use getopts::Options;
// use std::env;

// fn main_optopt() -> Result<(), Box<dyn Error>>{
//     let args: Vec<String> = env::args().collect();
//     // let program = args[0].clone();

//     let mut opts = Options::new();
//     opts.optopt("", "proto", "transport layer protocol to use", "tcp");
//     opts.optopt("", "port", "port to listen on or connect to", "5001");
//     opts.optopt("a", "address", "address or hostname of server to connect to", "5001");
//     opts.optflag("s", "server", "whether this is an echo server");
//     opts.optflag("c", "client", "whether this is an echo client");
//     let matches = opts.parse(&args[1..])?;
//     println!("{:?}", matches);
//     Ok(())
// }
//
