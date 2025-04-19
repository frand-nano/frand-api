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
    rocket_api_endpoint: String,
    
    // MongoDB 설정
    mongo_host: String,
    mongo_port: String,
    mongo_user: String,
    mongo_password: String,
    mongo_db_name: String,
}

impl ApiEnvConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = current_dir()?.join(path);
        
        // 환경 변수 파일 로드 시도
        dotenvy::from_path(path.as_path())
            .with_context(|| format!("환경 변수 파일을 로드하지 못했습니다: {}", path.display()))?;
        
        Ok(Self {
            log_level: env::var("LOG_LEVEL")?,
            rocket_api_endpoint: env::var("ROCKET_API_ENDPOINT")?,
            mongo_host: env::var("MONGO_HOST")?,
            mongo_port: env::var("MONGO_PORT")?,
            mongo_user: env::var("MONGO_USER")?,
            mongo_password: env::var("MONGO_PASSWORD")?,
            mongo_db_name: env::var("MONGO_DB_NAME")?,
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

    pub fn rocket_api_endpoint(&self) -> &str {
        &self.rocket_api_endpoint
    }

    pub fn mongo_uri(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.mongo_user, self.mongo_password, self.mongo_host, self.mongo_port,
        )
    }

    pub fn mongo_db_name(&self) -> &str {
        &self.mongo_db_name
    }
}