use std::process;

fn main() {
    // Configuration.
    let config = allnations::config::Config::new();
    // Run.
    if let Err(e) = allnations::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
