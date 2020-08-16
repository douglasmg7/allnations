use log::info;

pub struct Filter {
    pub min_price: i32, // Price x 100.
    pub max_price: i32, // Price x 100.
}

pub struct Config {
    pub run_mode: super::RunMode,
    pub db_filename: String,
    pub filter: Filter,
}

impl Config {
    pub fn new() -> Config {
        let mut db_filename =
            std::env::var("ZUNKA_ALLNATIONS_DB").expect("Environment variable ZUNKA_ALLNATIONS_DB");
        let run_mode: super::RunMode;

        // Set run mode and db filename.
        if cfg!(test) {
            run_mode = super::RunMode::Test();
            db_filename.push_str("-test");
        } else {
            if std::env::var("RUN_MODE")
                .unwrap_or("".to_string())
                .to_lowercase()
                == "production"
            {
                run_mode = super::RunMode::Prod();
            } else {
                run_mode = super::RunMode::Dev();
                db_filename.push_str("-dev");
            }
        };

        // Check if db exist.
        std::fs::metadata(&db_filename).expect(&format!("Db file not exit: {}", db_filename));

        Config {
            run_mode: run_mode,
            db_filename: db_filename,
            filter: Filter {
                min_price: 1000 * 100,
                max_price: 1_000_000 * 100,
            },
        }
    }

    // Log configuration.
    pub fn log(&self) {
        // Print run mode and version.
        info!(
            "Running in {} mode (version {})",
            match self.run_mode {
                super::RunMode::Prod() => "production",
                super::RunMode::Dev() => "development",
                super::RunMode::Test() => "test",
            },
            std::env!("CARGO_PKG_VERSION")
        );
        info!("Using db: {}", self.db_filename);
    }
}
