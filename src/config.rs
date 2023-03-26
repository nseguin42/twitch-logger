use config::Config as BaseConfig;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use dirs::config_dir;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub env_prefix: Option<String>,
    pub env_file: Option<PathBuf>,
    pub channels: Option<HashMap<String, ChannelConfig>>,
    pub log_file: Option<PathBuf>,
    pub log_level: Option<String>,
    pub db_url: Option<String>,
    pub db_table: Option<String>,
}

impl Config {
    pub fn new(config_path: Option<String>) -> Self {
        let config = Self::load(config_path);

        Self {
            env_prefix: config.get("env_prefix").unwrap_or_default(),
            env_file: config.get("env_file").unwrap_or_default(),
            channels: config.get("channel").unwrap_or_default(),
            log_file: config.get("log_file").unwrap_or_default(),
            log_level: config.get("log_level").unwrap_or_default(),
            db_url: config.get("db_url").unwrap_or_default(),
            db_table: config.get("db_table").unwrap_or_default(),
        }
    }

    fn load(config_path: Option<String>) -> BaseConfig {
        let mut config = BaseConfig::builder();

        if let Some(config_path) = config_path {
            let path = PathBuf::from(config_path);
            config = config.add_source(config::File::from(path));
        } else if let Some(config_dir) = config_dir() {
            let path = config_dir.join("twitch-logger").join("config.toml");
            if path.exists() {
                config = config.add_source(config::File::from(path));
            } else {
                config = config.add_source(config::File::with_name("config"));
            }
        }

        config.build().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelConfig {}
