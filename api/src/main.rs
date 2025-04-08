use log::info;

#[rocket::main]
async fn main() {
    // .env 파일 로드
    dotenvy::dotenv().ok();
    
    // 로거 초기화
    simple_logger::init_with_level(log::Level::Info).expect("로거 초기화 실패");
    
    info!("서버 시작 중...");
    
    // Rocket 인스턴스 생성 및 실행
    let rocket = api::create_rocket();
    if let Err(e) = rocket.launch().await {
        println!("서버 실행 중 오류 발생: {}", e);
    }
}
