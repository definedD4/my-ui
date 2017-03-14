use log::*;

struct MyLogger {
    max_log_level: LogLevel,
}

impl Log for MyLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.max_log_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

pub fn init(log_level: LogLevel) -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
        max_log_level.set(log_level.to_log_level_filter());
        Box::new(MyLogger {
            max_log_level: log_level,
        })
    })
}