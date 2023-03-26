use crate::config::Config;
use crate::error::Error;
use crate::logger::value_logger::ValueLogger;
use log::error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct FileLogger {
    pub path: PathBuf,
}

impl FileLogger {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn append_line(&self, line: &str) {
        // assume the file exists already.
        let mut file = OpenOptions::new().append(true).open(&self.path).unwrap();
        let line = format!("{}\n", line);

        if let Err(e) = file.write_all(line.as_bytes()) {
            error!("Error writing to file: {}", e);
        }

        file.flush().unwrap();
    }
}

impl ValueLogger<&str> for FileLogger {
    fn log(&mut self, message: &str) {
        self.append_line(message);
    }
}

impl TryFrom<&Config> for FileLogger {
    type Error = Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        if config.log_file.is_none() {
            return Err(Error::MissingConfig("log_file".to_string()));
        }

        let path = config.log_file.clone().unwrap();
        Ok(Self::new(path))
    }
}
