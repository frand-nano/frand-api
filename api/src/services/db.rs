use crate::config::AppConfig;
use log::{error, info};
use mongodb::{Client, Database, options::ClientOptions};
use std::error::Error;

/// MongoDB 데이터베이스 인스턴스 초기화
pub async fn init_db() -> Result<Database, Box<dyn Error>> {
    // 설정 로드
    let config = AppConfig::from_env();
    info!("MongoDB 연결 중: {}", config.db_uri);

    // 클라이언트 옵션 파싱
    let mut client_options = ClientOptions::parse(&config.db_uri).await?;
    client_options.app_name = Some("frand-api".to_string());

    // MongoDB 클라이언트 생성
    let client = match Client::with_options(client_options) {
        Ok(client) => {
            info!("MongoDB 클라이언트 생성 성공");
            client
        }
        Err(e) => {
            error!("MongoDB 클라이언트 생성 실패: {}", e);
            return Err(Box::new(e));
        }
    };

    // 데이터베이스 인스턴스 반환
    info!("데이터베이스 '{}' 사용 중", config.db_name);
    Ok(client.database(&config.db_name))
}
