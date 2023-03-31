use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub data_file: String,
    pub video_dir: String,
    pub audio_dir: String,
    pub video_cmd: String,
    pub audio_cmd: String,
}

#[derive(Debug)]
pub enum ConfigError {
    IOError(std::io::Error),
    YamlError(serde_yaml::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IOError(error) => write!(f, "IOError: {}", error),
            ConfigError::YamlError(error) => write!(f, "YamlError: {}", error),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::IOError(error)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError::YamlError(error)
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IOError(error) => Some(error),
            ConfigError::YamlError(error) => Some(error),
        }
    }
}

impl Config {
    pub fn build(file_name: &str) -> Result<Config, ConfigError> {
        let file_data = fs::read_to_string(file_name)?;
        let yaml: Config = serde_yaml::from_str(file_data.as_str())?;
        Ok(yaml)
    }
}
