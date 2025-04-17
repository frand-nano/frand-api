use mongodb::{options::ClientOptions, Client as MongoClient};
use rocket::local::asynchronous::Client;
use rocket::http::Status;
use serde_json::Value;

mod common;

#[rocket::async_test]
async fn test_health_endpoint() {
    // 테스트 환경 설정
    common::setup();
    
    // 애플리케이션 설정 로드
    let config = api::config::Config::load();
    
    // MongoDB 연결 설정
    let db_name = std::env::var("DATABASE_NAME").unwrap_or_else(|_| "frand_api_db_test".to_string());
    let client_options = ClientOptions::parse(&config.mongodb_uri()).await.unwrap();
    let mongo_client = MongoClient::with_options(client_options).unwrap();
    let db = mongo_client.database(&db_name);
    
    // Rocket 인스턴스 생성 및 MongoDB 관리
    let rocket = api::build_rocket(config).manage(db);
    
    // 테스트 클라이언트 생성 (비동기)
    let client = Client::tracked(rocket).await.expect("유효한 Rocket 인스턴스");
    
    // 헬스체크 경로 GET 요청 테스트
    let response = client.get("/api/v1/health").dispatch().await;
    
    // 상태 코드 검증
    assert_eq!(response.status(), Status::Ok);
    
    // 응답 형식 검증 (JSON)
    let content_type = response.content_type().unwrap();
    assert!(content_type.is_json());
    
    // 응답 본문 구조 검증
    let body_str = response.into_string().await.unwrap();
    let body: Value = serde_json::from_str(&body_str).expect("유효한 JSON");
    
    // 성공 플래그 검증
    assert_eq!(body["success"], true);
    
    // 데이터 필드 검증
    assert_eq!(body["data"]["status"], "ok");
}
