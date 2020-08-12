use chrono::Local;
use log;

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace))
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message: String;
            match record.level() {
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
            }
            println!("{}", message);

            // println!(
            // "[allnations] [{}] :{} -- {}",
            // record.level(),
            // record.target(),
            // record.args()
            // );
        }
    }
    fn flush(&self) {}
}
