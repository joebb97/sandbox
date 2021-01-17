extern crate getopts;
use clap::{Arg, App};
use echo_net::{run_server, run_client};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rust_echo_net")
        .version("1.0")
        .author("Joey Buiteweg <joebb@umich.edu>")
        .about("Create an echo server or client to send messages with.")
        .arg(Arg::new("proto")
             .long("proto")
             .value_name("tcp|udp")
             .default_value("tcp")
             .about("Set the transport layer protocol to either tcp or udp")
             .takes_value(true))
        .arg(Arg::new("port")
             .long("port")
             .value_name("PORT")
             .default_value("5005")
             .about("Set port to connect to/listen on")
             .takes_value(true))
        .arg(Arg::new("buffersize")
             .long("buffersize")
             .value_name("BUFSIZE")
             .default_value("1024")
             .about("Set size of buffer for communication")
             .takes_value(true))
        .arg(Arg::new("address")
             .long("addr")
             .short('a')
             .value_name("ADDR")
             .default_value("localhost")
             .about("Set IP or hostname to connect to")
             .takes_value(true))
        .arg(Arg::new("server")
             .short('s')
             .long("server")
             .takes_value(false)
             .about("Specify whether this an echo server"))
        .arg(Arg::new("client")
             .short('c')
             .long("client")
             .takes_value(false)
             .about("Specify whether this an echo client"))
        .get_matches();
    let server_set = matches.is_present("server");
    let client_set = matches.is_present("client");
    if !(server_set || client_set) || (server_set && client_set) {
        Err("Exactly one of -s|-c must be specified")?;
    }
    if server_set {
        run_server(matches)?
    } else if client_set {
        run_client(matches)?
    }
    Ok(())
}
