pub struct Config {
    pub run_mode: super::RunMode,
    pub db_filename: String,
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
            let mut args = std::env::args();
            // Prgram name.
            args.next();
            match args.next() {
                Some(arg) => {
                    if arg == "-p" {
                        run_mode = super::RunMode::Prod();
                    } else {
                        run_mode = super::RunMode::Dev();
                        db_filename.push_str("-dev");
                    }
                }
                None => {
                    run_mode = super::RunMode::Dev();
                    db_filename.push_str("-dev");
                }
            }
        };

        // Check if db exist.
        std::fs::metadata(&db_filename).expect(&format!("Db file not exit: {}", db_filename));

        Config {
            run_mode: run_mode,
            db_filename: db_filename,
        }
    }

    // Log configuration.
    pub fn log(&self) {
        // Print run mode and version.
        println!(
            "Running in {} mode (version {})",
            match self.run_mode {
                super::RunMode::Prod() => "production",
                super::RunMode::Dev() => "development",
                super::RunMode::Test() => "test",
            },
            std::env!("CARGO_PKG_VERSION")
        );
        println!("Using db: {}", self.db_filename);
    }
}
