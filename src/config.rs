pub struct Config {
    pub production: bool,
    pub db_filename: String,
}

impl Config {
    pub fn new() -> Config {
        let mut args = std::env::args();
        // Prgram name.
        args.next();

        // Query
        let production = match args.next() {
            Some(arg) => {
                if arg == "-p" {
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        let db_filename =
            std::env::var("ZUNKA_ALLNATIONS_DB").expect("Environment variable ZUNKA_ALLNATIONS_DB");

        Config {
            production,
            db_filename,
        }
    }

    // Log configuration.
    pub fn log(&self) {
        // Print run mode and version.
        println!(
            "Running in {} mode (version {})",
            if self.production {
                "production"
            } else {
                "development"
            },
            std::env!("CARGO_PKG_VERSION")
        );
        println!("Using db: {}", self.db_filename);
    }
}
