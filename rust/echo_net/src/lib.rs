use clap::ArgMatches;
use std::error::Error;

pub fn run_server(app: ArgMatches) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn run_client(app: ArgMatches) -> Result<(), Box<dyn Error>> {
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
