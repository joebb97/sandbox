use std::env;
use std::process;

use lc2k_as::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = lc2k_as::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
