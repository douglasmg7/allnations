use once_cell::sync::OnceCell;

// Production mode.
static PRODUCTION: OnceCell<bool> = OnceCell::new();

pub mod xml;

// Set run mode.
pub fn set_run_mode() {
    // Set run mode.
    match std::env::var_os("RUN_MODE") {
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
        std::env!("CARGO_PKG_VERSION")
    );
}

// If in production mode.
pub fn is_production() -> bool {
    if *PRODUCTION.get().expect("Run mode not defined") {
        return true;
    } else {
        return false;
    }
}