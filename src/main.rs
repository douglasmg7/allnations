use log::error;
// use std::fs::File;
use std::process;

fn main() {
    allnations::logger::init().unwrap();

    // Configuration.
    let config = allnations::config::Config::new();
    // Run.
    let stdin = std::io::stdin();
    if let Err(e) = allnations::run(config, stdin.lock()) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}
