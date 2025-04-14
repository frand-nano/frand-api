use api::config::Config;
use api::logger;
use log::info;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // .env 파일 로드
    dotenvy::dotenv().ok();
    
    // 설정 로드
    let config = Config::load();
    
    // 로거 초기화
    if let Err(e) = logger::init_logger(&config.log_level) {
        eprintln!("로거 초기화 실패: {}", e);
    }
    
    info!("서버 시작 중 - 주소: {}:{}", config.rocket_address, config.rocket_port);
    
    // Rocket 인스턴스 생성 및 실행
    let rocket = api::build_rocket(config);
    rocket.launch().await?;
    
    Ok(())
}
