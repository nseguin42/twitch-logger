use crate::logger::value_logger::ValueLogger;
use log::{debug, error, info, trace, warn, Level};

pub struct ConsoleLogger {
    pub level: Level,
}

impl ConsoleLogger {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

impl ValueLogger<&str> for ConsoleLogger {
    fn log(&self, msg: &str) {
        match self.level {
            Level::Error => error!("{}", msg),
            Level::Warn => warn!("{}", msg),
            Level::Info => info!("{}", msg),
            Level::Debug => debug!("{}", msg),
            Level::Trace => trace!("{}", msg),
        }
    }
}
