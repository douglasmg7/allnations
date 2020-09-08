use log::info;

#[derive(Clone)]
pub struct Filter {
    pub min_price: u32, // Price x 100.
    pub max_price: u32, // Price x 100.
}

#[derive(Clone)]
pub struct Config {
    pub run_mode: super::RunMode,
    pub zunkasite_host: String,
    pub zunkasite_user: String,
    pub zunkasite_pass: String,
    pub db_filename: String,
    pub filter: Filter,
}

impl Config {
    pub fn new() -> Config {
        let mut db_filename =
            std::env::var("ALLNATIONS_DB").expect("Environment variable ALLNATIONS_DB");
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

        // Zunkasite crendentials.
        let run_mode_sufix;
        match run_mode {
            super::RunMode::Prod() => {
                run_mode_sufix = "PROD";
            }
            _ => {
                run_mode_sufix = "DEV";
            }
        }
        let zunkasite_host =
            std::env::var(format!("ZUNKASITE_HOST_{}", run_mode_sufix)).expect(&format!(
                "Environment variable ZUNKASITE_HOST_{} not defined",
                run_mode_sufix
            ));
        let zunkasite_user =
            std::env::var(format!("ZUNKASITE_USER_{}", run_mode_sufix)).expect(&format!(
                "Environment variable ZUNKASITE_USER_{} not defined",
                run_mode_sufix
            ));
        let zunkasite_pass =
            std::env::var(format!("ZUNKASITE_PASS_{}", run_mode_sufix)).expect(&format!(
                "Environment variable ZUNKASITE_PASS_{} not defined",
                run_mode_sufix
            ));

        // Check if db exist.
        std::fs::metadata(&db_filename).expect(&format!("Db file not exit: {}", db_filename));

        Config {
            run_mode: run_mode,
            zunkasite_host: zunkasite_host,
            zunkasite_user: zunkasite_user,
            zunkasite_pass: zunkasite_pass,
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
