use anyhow::Result;
use serde::Deserialize;
use std::{env::current_dir, path::Path};
use config::{Config, File};

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    log_level: String,
    pub server: ApiServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ApiServerConfig {
    pub port: u16,
    pub host: String,
    pub api_version: String,
}

impl ApiConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = current_dir()?.join(path);
        
        let config = Config::builder()
            .add_source(File::from(path))
            .build()?;
        
        Ok(config.try_deserialize()?)
    }

    pub fn log_level(&self) -> log::Level {
        match self.log_level.as_str() {
            "error" => log::Level::Error,
            "warn" => log::Level::Warn,
            "info" => log::Level::Info,
            "debug" => log::Level::Debug,
            "trace" => log::Level::Trace,
            _ => log::Level::Info,
        }
    }
}