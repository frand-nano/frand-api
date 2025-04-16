use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::str::FromStr;

// 로거 초기화 함수
pub fn init_logger(log_level: &str) -> Result<(), log::SetLoggerError> {
    let level = LevelFilter::from_str(log_level).unwrap_or(LevelFilter::Info);
    SimpleLogger::new().with_level(level).init()
}
