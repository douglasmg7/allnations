use super::RunMode;
use chrono::Local;
use log;

pub struct Logger<'a> {
    run_mode: &'a RunMode,
}

impl log::Log for Logger<'_> {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message: String;
            match self.run_mode {
                RunMode::Dev() | RunMode::Test() => match record.level() {
                    log::Level::Error => {
                        message = format!("[error] {}", record.args());
                    }
                    log::Level::Warn => {
                        message = format!("[warn] {}", record.args());
                    }
                    log::Level::Info => {
                        message = format!("{}", record.args());
                    }
                    log::Level::Debug => {
                        message = format!("[debug] {}", record.args());
                    }
                    log::Level::Trace => {
                        message = format!("[trace] {}", record.args());
                    }
                },
                RunMode::Prod() => match record.level() {
                    log::Level::Error => {
                        message = format!(
                            "{} [{}] [error] {}",
                            Local::now().format("%Y/%m/%d %H:%M:%S%.6f"),
                            record.target(),
                            record.args()
                        );
                    }
                    log::Level::Warn => {
                        message = format!(
                            "{} [{}] [warn] {}",
                            Local::now().format("%Y/%m/%d %H:%M:%S%.6f"),
                            record.target(),
                            record.args()
                        );
                    }
                    log::Level::Info => {
                        message = format!(
                            "{} [{}] [info] {}",
                            Local::now().format("%Y/%m/%d %H:%M:%S%.6f"),
                            record.target(),
                            record.args()
                        );
                    }
                    log::Level::Debug => {
                        message = format!(
                            "{} [{}] [debug] {}",
                            Local::now().format("%Y/%m/%d %H:%M:%S%.6f"),
                            record.target(),
                            record.args()
                        );
                    }
                    log::Level::Trace => {
                        message = format!(
                            "{} [{}] [trace] {}",
                            Local::now().format("%Y/%m/%d %H:%M:%S%.6f"),
                            record.target(),
                            record.args()
                        );
                    }
                },
            }
            println!("{}", message);
        }
    }
    fn flush(&self) {}
}

pub fn init(run_mode: &'static RunMode) -> Result<(), log::SetLoggerError> {
    let logger = Logger { run_mode: run_mode };

    let r = log::set_boxed_logger(Box::new(logger));
    if r.is_ok() {
        log::set_max_level(log::LevelFilter::Trace);
    }
    r
}
