use crate::config::Config;
use crate::error::Error;
use async_trait::async_trait;

use chrono::serde::ts_seconds::deserialize as from_ts_seconds;
use chrono::{DateTime, Utc};
use log::debug;
use serde::Deserialize;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use twitch_irc::login::{TokenStorage, UserAccessToken};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

#[derive(Debug, Clone)]
pub struct EnvStorage {
    env_file_path: Option<PathBuf>,
    prefix: Option<String>,
}

impl EnvStorage {
    pub fn new(env_file_path: Option<PathBuf>, prefix: Option<String>) -> Self {
        if let Some(prefix) = &prefix {
            if prefix.contains('_') {
                panic!("Prefix cannot contain underscores");
            }
        }

        if let Some(path) = &env_file_path {
            if path.exists() {
                dotenv::from_path(path).expect("Failed to load env file");
            } else {
                std::fs::File::create(path).expect("Failed to create env file");
            }
        }

        Self {
            env_file_path,
            prefix,
        }
    }

    fn load_env_file(&self) -> Result<(), Error> {
        if let Some(path) = &self.env_file_path {
            dotenv::from_path(path)
                .map_err(|e| Error::MissingEnv(format!("Failed to load env file: {}", e)))?;
        }
        Ok(())
    }

    pub fn get_env<T>(&self, key: &str) -> Result<T, Error>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.get_env_opt(key)?
            .ok_or_else(|| Error::MissingEnv(self.format_key(key)))
    }

    pub fn get_env_opt<T>(&self, key: &str) -> Result<Option<T>, Error>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let key = if let Some(prefix) = &self.prefix {
            format!("{}_{}", prefix, key)
        } else {
            key.to_string()
        };

        let value = std::env::var(&key).map_err(|_e| Error::MissingEnv(key.clone()))?;
        debug!("Found env: {}={}", key, value);

        let value = value.parse::<T>().map_err(|e| Error::FailedToParse {
            key: key.to_string(),
            value: value.to_string(),
            error: Some(format!("{:?}", e)),
        })?;

        Ok(Some(value))
    }

    fn format_key(&self, key: &str) -> String {
        let key = key.to_uppercase();
        if let Some(prefix) = &self.prefix {
            format!("{}_{}", prefix, key)
        } else {
            key
        }
    }

    pub fn set_env<T>(&self, key: &str, value: T) -> Result<(), Error>
    where
        T: ToString,
    {
        let key = self.format_key(key);
        let value = value.to_string();
        std::env::set_var(&key, &value);
        self.update_env_file(&key, &value)?;

        Ok(())
    }

    fn escape(value: &str) -> String {
        value.replace('\"', "\\\"")
    }

    fn get_formatted_line(&self, key: &str, value: &str) -> String {
        format!("{}=\"{}\"", Self::escape(key), Self::escape(value))
    }

    fn update_env_file(&self, key: &str, value: &str) -> Result<(), Error> {
        if self.env_file_path.is_none() {
            return Ok(());
        }

        let new_line = self.get_formatted_line(key, value);

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(self.env_file_path.as_ref().unwrap())
            .map_err(|e| Error::MissingEnv(format!("Failed to open env file: {}", e)))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| Error::MissingEnv(format!("Failed to read env file: {}", e)))?;

        let mut lines = contents.lines().collect::<Vec<_>>();
        let mut found = false;
        for line in lines.iter_mut() {
            if line.starts_with(key) {
                debug!("Updating env file: \"{}\" -> \"{}\"", line, new_line);
                *line = &new_line;
                found = true;
            }
        }

        if !found {
            debug!("Appending to env file: \"{}\"", new_line);
            lines.push(&new_line);
        }

        let contents = lines.join(LINE_ENDING);
        overwrite_file(self.env_file_path.as_ref().unwrap(), &contents)?;
        Ok(())
    }
}

fn overwrite_file(path: &Path, contents: &str) -> Result<(), Error> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .map_err(|e| Error::MissingEnv(format!("Failed to open env file: {}", e)))?;

    file.write_all(contents.as_bytes())
        .map_err(|e| Error::MissingEnv(format!("Failed to write env file: {}", e)))?;

    Ok(())
}

impl From<&Config> for EnvStorage {
    fn from(config: &Config) -> Self {
        Self::new(config.env_file.clone(), config.env_prefix.clone())
    }
}

#[async_trait]
impl TokenStorage for EnvStorage {
    type LoadError = std::io::Error; // or some other error
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        // serde deserialize with seconds
        let token = Token {
            access_token: self.get_env("ACCESS_TOKEN").unwrap(),
            refresh_token: self.get_env("REFRESH_TOKEN").unwrap(),
            created_at: self.get_env("TOKEN_CREATED_AT").unwrap_or_default(),
            expires_at: self.get_env("TOKEN_EXPIRES_AT").unwrap_or_default(),
        };

        Ok(UserAccessToken {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            created_at: token.created_at,
            expires_at: Some(token.expires_at),
        })
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        self.set_env("ACCESS_TOKEN", &token.access_token).unwrap();
        self.set_env("REFRESH_TOKEN", &token.refresh_token).unwrap();
        self.set_env("TOKEN_CREATED_AT", token.created_at).unwrap();
        self.set_env("TOKEN_EXPIRES_AT", token.expires_at.unwrap_or_default())
            .unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Token {
    access_token: String,
    refresh_token: String,
    #[serde(deserialize_with = "from_ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(deserialize_with = "from_ts_seconds")]
    expires_at: DateTime<Utc>,
}
