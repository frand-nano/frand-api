mod common;
use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn test_root_endpoint() {
    // 테스트 환경 설정
    common::setup();
    
    // Rocket 인스턴스 생성
    let rocket = api::create_rocket();
    let client = Client::tracked(rocket).expect("유효한 Rocket 인스턴스");
    
    // 루트 경로 테스트
    let response = client.get("/").dispatch();
    
    // 응답 검증
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "hello world");
}
