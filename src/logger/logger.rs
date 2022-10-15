use chrono::{DateTime, Utc};
use log;
use log::{LevelFilter, Log, Metadata, Record};

struct Logger {
    log_level: LevelFilter,
}

static mut LOGGER: Logger = Logger {
    log_level: LevelFilter::Off,
};

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // if level filter less than or equal to incoming record then return true
        if metadata.level().to_level_filter() <= self.log_level {
            return true;
        } else {
            return false;
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
                _ => "[UNKNOWN]",
            };

            // print formatted message to stdout
            // e.g. [!] Sun Jul 8 00:34:60 2001 : "Hello, World!"
            println!("{} {} : \"{}\"", prefix, timestamp, message)
        }
    }

    fn flush(&self) {
        unimplemented!()
    }
}

pub fn init(level: LevelFilter) {
    unsafe {
        // change log level from default
        LOGGER.log_level = level;

        // pass static logger to log library
        log::set_logger(&LOGGER).expect("Unable to setup logger");
        log::set_max_level(LOGGER.log_level);
    }
}
