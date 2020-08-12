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
                    message = format!("[{}] [error] {}", record.target(), record.args());
                }
                log::Level::Warn => {
                    message = format!("[{}] [warn] {}", record.target(), record.args());
                }
                log::Level::Info => {
                    message = format!(
                        "{} [{}] [info] {}",
                        Local::now().format("%Y-%m-%dT%H:%M:%S"),
                        record.target(),
                        record.args()
                    );
                }
                log::Level::Debug => {
                    message = format!("[{}] [debug] {}", record.target(), record.args());
                }
                log::Level::Trace => {
                    message = format!("[{}] [trace] {}", record.target(), record.args());
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
