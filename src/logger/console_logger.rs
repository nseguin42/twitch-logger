use crate::config::Config;
use crate::error::Error;
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
    fn log(&mut self, msg: &str) {
        match self.level {
            Level::Error => error!("{}", msg),
            Level::Warn => warn!("{}", msg),
            Level::Info => info!("{}", msg),
            Level::Debug => debug!("{}", msg),
            Level::Trace => trace!("{}", msg),
        }
    }
}

impl TryFrom<&Config> for ConsoleLogger {
    type Error = Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        if config.log_level.is_none() {
            return Err(Error::MissingConfig("log_level".to_string()));
        }

        let level = config.log_level.clone().unwrap();
        let level = match level.to_lowercase().as_str() {
            "error" => Level::Error,
            "warn" => Level::Warn,
            "info" => Level::Info,
            "debug" => Level::Debug,
            "trace" => Level::Trace,
            _ => return Err(Error::InvalidConfig("log_level".to_string())),
        };

        Ok(Self::new(level))
    }
}
