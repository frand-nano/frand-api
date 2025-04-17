use api::config::Config;
use api::logger;
use log::info;
use mongodb::{options::ClientOptions, Client};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env 파일 로드
    dotenvy::dotenv().ok();
    
    // 설정 로드
    let config = Config::load();
    
    // 로거 초기화
    if let Err(e) = logger::init_logger(&config.log_level) {
        eprintln!("로거 초기화 실패: {}", e);
    }
    
    // MongoDB 클라이언트 초기화
    let client_options = ClientOptions::parse(&config.mongodb_uri()).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&config.database_name);
    
    info!("MongoDB 연결 성공: {}", config.database_name);
    info!("서버 시작 중 - 주소: {}:{}", config.rocket_address, config.rocket_port);
    
    // Rocket 인스턴스 생성 및 실행
    let rocket = api::build_rocket(config).manage(db);
    rocket.launch().await?;
    
    Ok(())
}
