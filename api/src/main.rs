use log::{error, info};

#[rocket::main]
async fn main() {
    // .env 파일 로드
    dotenvy::dotenv().ok();
    
    // 로거 초기화
    simple_logger::init_with_level(log::Level::Info).expect("로거 초기화 실패");
    
    info!("서버 시작 중...");
    
    // MongoDB 데이터베이스 인스턴스 초기화
    let db = match api::services::db::init_db().await {
        Ok(db) => {
            info!("MongoDB 연결 성공");
            db
        },
        Err(e) => {
            error!("MongoDB 연결 실패: {}", e);
            panic!("MongoDB 연결 실패");
        }
    };
    
    // Rocket 인스턴스 생성 및 실행
    let rocket = api::create_rocket(db);
    if let Err(e) = rocket.launch().await {
        error!("서버 실행 중 오류 발생: {}", e);
    }
}
