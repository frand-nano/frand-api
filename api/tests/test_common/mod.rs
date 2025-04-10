use mongodb::{Database, Client as MongoClient, options::ClientOptions};
use api::config::AppConfig;
use api::models::DBItem;
use rocket::local::asynchronous::Client;

pub fn load_env_test() {
    // .env.test 파일 로드 (이미 로드되었더라도 오류를 반환하지 않음)
    dotenvy::from_filename(".env.test").ok();
}

/// 테스트 전 데이터베이스 정리
async fn clean_db(db: &Database) {
    let _ = db.collection::<DBItem>("items").drop(None).await;
}

/// 테스트 데이터베이스 생성
pub async fn setup_test_db() -> Database {        
    let config = AppConfig::from_env();

    let client_options = ClientOptions::parse(&config.db_uri).await.expect("DB URI 파싱 실패");
    let client = MongoClient::with_options(client_options).expect("MongoDB 클라이언트 생성 실패");
    
    let db = client.database(&config.db_name);
    clean_db(&db).await;

    db
}

/// 테스트 클라이언트 생성
pub async fn setup_test_client(db: Database) -> Client {
    // Rocket 인스턴스 생성
    let rocket = api::create_rocket(db);
    Client::tracked(rocket).await.expect("유효한 Rocket 인스턴스")
}
