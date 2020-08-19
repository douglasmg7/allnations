use super::RunMode;
use chrono::Local;
use lazy_static::lazy_static;
use log;

struct Logger {
    run_mode: RunMode,
}

static NUM: i32 = 3;

static LOGGER: Logger = Logger {
    run_mode: RunMode::Dev(),
};

// lazy_static! {
// static ref LOGGER: Logger = {
// let lg = Logger {
// run_mode: RunMode::Dev(),
// };
// lg
// };
// }

pub fn init() -> Result<(), log::SetLoggerError> {
    // lazy_static! {
    // static ref LOGGER: Logger = {
    // let lg = Logger {
    // run_mode: RunMode::Dev(),
    // };
    // lg
    // };
    // }
    println!("NUM: {}", NUM);
    LOGGER::test();
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace))
}

impl Logger {
    fn test(&self) {
        match self.run_mode {
            RunMode::Dev() => {
                println!("Test");
            }
            _ => {}
        }
    }
}

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
                log::Level::Info => match self.run_mode {
                    RunMode::Dev() | RunMode::Test() => {
                        message = format!("{}", record.args());
                    }
                    _ => {
                        message = format!(
                            "{} [{}] [info] {}",
                            Local::now().format("%Y/%m/%d %H:%M:%S%.6f"),
                            record.target(),
                            record.args()
                        );
                    }
                },
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
