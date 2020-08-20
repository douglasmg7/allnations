use allnations::config::Config;
use lazy_static::lazy_static;
use log::error;
use std::process;

fn main() {
    // Configuration.
    lazy_static! {
        static ref CONFIG: Config = Config::new();
    }

    // Init log.
    allnations::logger::init(&CONFIG.run_mode).unwrap();

    // Run.
    let stdin = std::io::stdin();
    if let Err(e) = allnations::run(&CONFIG, stdin.lock()) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}
