use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use std::env::{self, current_dir};
use anyhow::Context; 

#[derive(Debug, Deserialize)]
pub struct ApiEnvConfig {
    // 로그 설정
    log_level: String,
    
    // 로켓 웹서버 설정
    pub rocket_address: String,
    pub rocket_port: String,
    pub rocket_api_endpoint: String,
}

impl ApiEnvConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = current_dir()?.join(path);
        
        // 환경 변수 파일 로드 시도
        dotenvy::from_path(path.as_path())
            .with_context(|| format!("환경 변수 파일을 로드하지 못했습니다: {}", path.display()))?;
        
        Ok(Self {
            log_level: env::var("LOG_LEVEL")?,
            rocket_address: env::var("ROCKET_ADDRESS")?,
            rocket_port: env::var("ROCKET_PORT")?,
            rocket_api_endpoint: env::var("ROCKET_API_ENDPOINT")?,
        })
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