use once_cell::sync::OnceCell;
use std::env;
// use std::io::prelude::*;
use std::io::{self, Read};

static PRODUCTION: OnceCell<bool> = OnceCell::new();

pub fn set_run_mode() {
    // Set run mode.
    match env::var_os("RUN_MODE") {
        Some(val) => {
            if val == "production" {
                PRODUCTION.set(true).unwrap();
            } else {
                PRODUCTION.set(false).unwrap();
            }
        }
        None => {
            PRODUCTION.set(false).unwrap();
        }
    }
    // Print run mode and version.
    println!(
        "Runing in {} mode (version {})",
        if is_production() {
            "production"
        } else {
            "development"
        },
        env!("CARGO_PKG_VERSION")
    );
}

pub fn is_production() -> bool {
    if *PRODUCTION.get().expect("Run mode not defined") {
        return true;
    } else {
        return false;
    }
}

pub fn read_test() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    // println!("stdin: {}", buffer);
    Ok(())
}