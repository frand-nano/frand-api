use rocket::local::blocking::Client;
use rocket::http::Status;

mod common;

#[test]
fn test_root_endpoint() {
    // 테스트 환경 설정
    common::setup();
    
    // 애플리케이션 설정 로드
    let config = api::config::Config::load();
    
    // Rocket 인스턴스 생성
    let rocket = api::build_rocket(config);
    
    // 테스트 클라이언트 생성
    let client = Client::tracked(rocket).expect("유효한 Rocket 인스턴스");
    
    // 루트 경로 GET 요청 테스트
    let response = client.get("/").dispatch();
    
    // 상태 코드 검증
    assert_eq!(response.status(), Status::Ok);
    
    // 응답 본문 검증
    assert_eq!(response.into_string().unwrap(), "hello world");
}
