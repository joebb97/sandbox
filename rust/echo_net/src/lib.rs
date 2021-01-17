use clap::ArgMatches;
use std::thread;
use std::error::Error;
use std::net::{TcpListener, TcpStream, Shutdown, UdpSocket};
use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::str::from_utf8;

fn handle_connection_tcp(mut stream: TcpStream, buf_size: usize) -> Result<(), Box<dyn Error>> {
    let mut data = vec![0 as u8; buf_size];
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
            stream.shutdown(Shutdown::Both)?;
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
    let proto: &str = app.value_of("protocol").ok_or("No proto")?;
    if proto == "tcp" {
        let listener = TcpListener::bind(combined)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Accepted connection {:?}", stream);
                    thread::spawn(move || {
                        let _ = handle_connection_tcp(stream, buf_size);
                    });
                }
                Err(e) => { 
                    println!("an error");
                    Err(e)?
                }
            }
        }
        // This won't be reached unless we handle a signal or set nonblocking.
    } else {
        let socket = UdpSocket::bind(combined)?;
        let mut buf: Vec<u8> = vec![0 as u8; buf_size];
        loop {
            let (amt, src) = socket.recv_from(&mut buf)?;
            println!("{}", from_utf8(&buf[..amt])?);
            socket.send_to(&buf[..amt], &src)?;
        }
    }
    Ok(())
}

pub fn run_client(app: ArgMatches) -> Result<(), Box<dyn Error>> {
    let addr: &str = app.value_of("address").ok_or("No address")?;
    let port: &str = app.value_of("port").ok_or("No port")?;
    let combined = format!("{}:{}", addr, port);
    let stdin = io::stdin();
    let proto: &str = app.value_of("protocol").ok_or("No proto")?;
    if proto == "tcp" {
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
    } else {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        for line in stdin.lock().lines() {
            let line_ref: &String = &line?;
            let bytes = line_ref.as_bytes();
            socket.send_to(bytes, &combined)?;
            let mut data: Vec<u8> = vec![0 as u8; bytes.len()];
            match socket.recv(&mut data) {
                Ok(_) => {
                    println!("{}", from_utf8(&data)?);
                }
                Err(e) => Err(e)?
            }
        }
    }
    Ok(())
}
