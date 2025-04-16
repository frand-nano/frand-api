use std::env;

// 애플리케이션 설정을 관리하는 구조체
pub struct Config {
    pub rocket_address: String,
    pub rocket_port: u16,
    pub log_level: String,
    // MongoDB 연결 정보 추가
    pub database_user: String,
    pub database_pass: String,
    pub database_host: String,
    pub database_port: u16,
}

impl Config {
    // 설정값 로드
    pub fn load() -> Self {
        // 로그 레벨 기본값 설정
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        
        // API 서버 주소 및 포트 설정
        let rocket_address = env::var("ROCKET_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
        let rocket_port = env::var("ROCKET_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("ROCKET_PORT 환경 변수는 유효한 숫자여야 합니다");

        // MongoDB 연결 정보 설정
        let database_user = env::var("DATABASE_USER").expect("DATABASE_USER 환경 변수가 설정되어야 합니다");
        let database_pass = env::var("DATABASE_PASS").expect("DATABASE_PASS 환경 변수가 설정되어야 합니다");
        let database_host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let database_port = env::var("DATABASE_PORT")
            .unwrap_or_else(|_| "27017".to_string())
            .parse::<u16>()
            .expect("DATABASE_PORT 환경 변수는 유효한 숫자여야 합니다");

        Self {
            rocket_address,
            rocket_port,
            log_level,
            database_user,
            database_pass,
            database_host,
            database_port,
        }
    }

    // MongoDB 연결 문자열 생성 메서드
    pub fn mongodb_uri(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.database_user, self.database_pass, self.database_host, self.database_port
        )
    }
}
