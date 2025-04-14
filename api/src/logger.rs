use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::str::FromStr;

// 로거 초기화 함수
pub fn init_logger(log_level: &str) -> Result<(), log::SetLoggerError> {
    // 문자열 로그 레벨을 LevelFilter로 변환
    let level = LevelFilter::from_str(log_level).unwrap_or(LevelFilter::Info);
    
    // 로거 설정 및 초기화
    SimpleLogger::new()
        .with_level(level)
        .init()
}
