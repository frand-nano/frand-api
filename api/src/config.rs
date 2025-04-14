use std::env;

// 애플리케이션 설정을 관리하는 구조체
pub struct Config {
    pub rocket_address: String,
    pub rocket_port: u16,
    pub log_level: String,
}

impl Config {
    // 설정값 로드
    pub fn load() -> Self {
        let rocket_address = env::var("ROCKET_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
        
        let rocket_port = env::var("ROCKET_PORT")
            .unwrap_or_else(|_| String::from("8000"))
            .parse::<u16>()
            .unwrap_or(8000);
        
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| String::from("info"));
        
        Config {
            rocket_address,
            rocket_port,
            log_level,
        }
    }
}
