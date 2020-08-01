use once_cell::sync::OnceCell;

// Production mode.
static PRODUCTION: OnceCell<bool> = OnceCell::new();
static DB_FILE: OnceCell<String> = OnceCell::new();

pub mod db;
pub mod product;

// Set run mode.
pub fn set_run_mode() {
    // Database location.
    DB_FILE
        .set(
            std::env::var("ZUNKA_ALLNATIONS_DB").expect("Environment variable ZUNKA_ALLNATIONS_DB"),
        )
        .unwrap();
    println!("Database location: {}", DB_FILE.get().unwrap());
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