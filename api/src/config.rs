use std::env;

/// 애플리케이션 설정 구조체
pub struct AppConfig {
    /// MongoDB 데이터베이스 이름
    pub db_name: String,
    /// MongoDB 연결 문자열
    pub db_uri: String,
}

impl AppConfig {
    /// 환경 변수에서 설정 로드
    pub fn from_env() -> Self {
        // MongoDB 연결 정보 로드
        let db_user = env::var("DATABASE_USER").unwrap_or_else(|_| "your_username".to_string());
        let db_pass = env::var("DATABASE_PASS").unwrap_or_else(|_| "your_password".to_string());
        let db_host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_port = env::var("DATABASE_PORT").unwrap_or_else(|_| "27017".to_string());
        let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "frand_api_db".to_string());

        // MongoDB 연결 URI 생성
        let db_uri = format!(
            "mongodb://{}:{}@{}:{}",
            db_user, db_pass, db_host, db_port
        );

        AppConfig { db_name, db_uri }
    }
}
