use chrono::{DateTime, Utc};
use log;
use log::{Record, Level, Metadata, LevelFilter, Log};

struct Logger {
    log_level: LevelFilter
}

static mut LOGGER: Logger = Logger { log_level: LevelFilter::Info };

impl Log for Logger {

    // Level: https://docs.rs/log/latest/src/log/lib.rs.html#425
    fn enabled(&self, metadata: &Metadata) -> bool {
        // if level filter less than or equal to incoming record then return true
        if metadata.level().to_level_filter() <= self.log_level {
            return true
        } else {
            return false
        }
    }

    fn log(&self, record: &Record) {
        // check if we want to print the record
        if self.enabled(record.metadata()) {

            // incoming record
            let message = record.args().to_string();

            let time: DateTime<Utc> = Utc::now();
            // "Sun Jul 8 00:34:60 2001"
            let timestamp = time.format("%c").to_string();


            // convert level to level_filter for matching
            let level_filter = record.level().to_level_filter();
            let prefix = match level_filter {
                LevelFilter::Error | LevelFilter::Warn => "[!]",
                LevelFilter::Info => "[*]",
                LevelFilter::Debug => "[DEBUG]",
                LevelFilter::Trace => "[TRACE]",
                _ => "[UNKNOWN]"
            };

            // print formatted message to stdout
            println!("{} {} : {}", prefix, timestamp, message)
        }
    }

    // flushing isn't needed
    fn flush(&self) {
        todo!()
    }
}

pub fn init(level: LevelFilter) {
    unsafe {
        // change log level from default
        LOGGER.log_level = level;

        // pass static logger to log library
        log::set_logger(&LOGGER).expect("Unable to setup logger");
        log::set_max_level(LOGGER.log_level)
    }
}