use mongodb::{options::ClientOptions, Client};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client as RocketClient;
use serde_json::Value;

mod common;

// 테스트 데이터베이스 초기화 및 정리
async fn setup_db() -> mongodb::Database {
    let config = api::config::Config::load();
    
    let client_options = ClientOptions::parse(&config.mongodb_uri()).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    client.database(&config.database_name)
}

#[rocket::async_test]
async fn test_create_memo_success() {
    // 테스트 환경 설정
    common::setup();

    // 환경 설정 및 Rocket 인스턴스 생성
    let config = api::config::Config::load();
    let db = setup_db().await;
    
    let rocket = api::build_rocket(config).manage(db);
    let client = RocketClient::tracked(rocket).await.expect("유효한 Rocket 인스턴스");
    
    // 메모 생성 테스트
    let response = client
        .post("/api/v1/memos")
        .header(ContentType::JSON)
        .body(r#"{"title":"테스트 메모","content":"테스트 내용"}"#)
        .dispatch()
        .await;
    
    // 응답 검증
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    
    let body_str = response.into_string().await.unwrap();
    println!("응답 JSON: {}", body_str); // 응답 JSON 출력하여 구조 확인
    
    let body: Value = serde_json::from_str(&body_str).expect("유효한 JSON");
    
    // 성공 응답 구조 확인
    assert_eq!(body["success"], true);
    
    // ObjectId가 문자열 대신 객체로 반환되는 경우를 처리
    if body["data"]["_id"].is_object() {
        assert!(body["data"]["_id"]["$oid"].is_string());
    } else {
        assert!(body["data"]["_id"].is_string());
    }
    
    assert_eq!(body["data"]["title"], "테스트 메모");
    assert_eq!(body["data"]["content"], "테스트 내용");
    
    // 타임스탬프가 객체로 반환되는 경우 처리
    if body["data"]["created_at"].is_object() {
        assert!(body["data"]["created_at"]["$date"].is_number());
    } else {
        assert!(body["data"]["created_at"].is_number());
    }
    
    if body["data"]["updated_at"].is_object() {
        assert!(body["data"]["updated_at"]["$date"].is_number());
    } else {
        assert!(body["data"]["updated_at"].is_number());
    }
}

#[rocket::async_test]
async fn test_validation_error() {
    // 테스트 환경 설정
    common::setup();
    
    // 환경 설정 및 Rocket 인스턴스 생성
    let config = api::config::Config::load();
    let db = setup_db().await;
    
    let rocket = api::build_rocket(config).manage(db);
    let client = RocketClient::tracked(rocket).await.expect("유효한 Rocket 인스턴스");
    
    // 길이 제한 위반 테스트 (빈 제목)
    let response = client
        .post("/api/v1/memos")
        .header(ContentType::JSON)
        .body(r#"{"title":"","content":"테스트 내용"}"#)
        .dispatch()
        .await;
    
    // 응답 검증 (400 Bad Request)
    assert_eq!(response.status(), Status::BadRequest);
    
    let body_str = response.into_string().await.unwrap();
    let body: Value = serde_json::from_str(&body_str).expect("유효한 JSON");
    
    // 오류 응답 구조 확인
    assert_eq!(body["success"], false);
    assert_eq!(body["error"]["code"], "VALIDATION_ERROR");
}

#[rocket::async_test]
async fn test_crud_flow() {
    // 테스트 환경 설정
    common::setup();
    
    // 환경 설정 및 Rocket 인스턴스 생성
    let config = api::config::Config::load();
    let db = setup_db().await;
    
    let rocket = api::build_rocket(config).manage(db);
    let client = RocketClient::tracked(rocket).await.expect("유효한 Rocket 인스턴스");
    
    // 1. 메모 생성
    let create_response = client
        .post("/api/v1/memos")
        .header(ContentType::JSON)
        .body(r#"{"title":"테스트 메모","content":"테스트 내용"}"#)
        .dispatch()
        .await;
    
    let create_body_str = create_response.into_string().await.unwrap();
    println!("생성 응답 JSON: {}", create_body_str);
    
    let create_body: Value = serde_json::from_str(&create_body_str).unwrap();
    
    // ID가 객체일 경우와 문자열일 경우 처리
    let memo_id = if create_body["data"]["_id"].is_object() {
        create_body["data"]["_id"]["$oid"].as_str().unwrap()
    } else {
        create_body["data"]["_id"].as_str().unwrap()
    };
    
    println!("메모 ID: {}", memo_id);
    
    // 이후 테스트 코드는 동일
    // 2. 메모 조회
    let get_response = client
        .get(format!("/api/v1/memos/{}", memo_id))
        .dispatch()
        .await;
    
    assert_eq!(get_response.status(), Status::Ok);
    let get_body: Value = serde_json::from_str(&get_response.into_string().await.unwrap()).unwrap();
    assert_eq!(get_body["data"]["title"], "테스트 메모");
    
    // 3. 메모 목록 조회
    let list_response = client.get("/api/v1/memos").dispatch().await;
    assert_eq!(list_response.status(), Status::Ok);
    let list_body: Value = serde_json::from_str(&list_response.into_string().await.unwrap()).unwrap();
    assert!(list_body["data"].as_array().unwrap().len() > 0);
    
    // 4. 메모 수정
    let update_response = client
        .put(format!("/api/v1/memos/{}", memo_id))
        .header(ContentType::JSON)
        .body(r#"{"title":"수정된 메모","content":"수정된 내용"}"#)
        .dispatch()
        .await;
    
    assert_eq!(update_response.status(), Status::Ok);
    let update_body: Value = serde_json::from_str(&update_response.into_string().await.unwrap()).unwrap();
    assert_eq!(update_body["data"]["title"], "수정된 메모");
    assert_eq!(update_body["data"]["content"], "수정된 내용");
    
    // 5. 메모 삭제
    let delete_response = client
        .delete(format!("/api/v1/memos/{}", memo_id))
        .dispatch()
        .await;
    
    assert_eq!(delete_response.status(), Status::Ok);
    
    // 6. 삭제 후 조회 시도 (404 확인)
    let get_deleted_response = client
        .get(format!("/api/v1/memos/{}", memo_id))
        .dispatch()
        .await;
    
    assert_eq!(get_deleted_response.status(), Status::NotFound);
}

#[rocket::async_test]
async fn test_not_found() {
    // 테스트 환경 설정
    common::setup();
    
    // 환경 설정 및 Rocket 인스턴스 생성
    let config = api::config::Config::load();
    let db = setup_db().await;
    
    let rocket = api::build_rocket(config).manage(db);
    let client = RocketClient::tracked(rocket).await.expect("유효한 Rocket 인스턴스");
    
    // 존재하지 않는 ID로 조회
    let invalid_id = "630a1f470d1d1ed3e972cfdd"; // 유효한 ObjectId 형식이지만 존재하지 않는 ID
    let response = client
        .get(format!("/api/v1/memos/{}", invalid_id))
        .dispatch()
        .await;
    
    assert_eq!(response.status(), Status::NotFound);
}

#[rocket::async_test]
async fn test_invalid_id_format() {
    // 테스트 환경 설정
    common::setup();
    
    // 환경 설정 및 Rocket 인스턴스 생성
    let config = api::config::Config::load();
    let db = setup_db().await;
    
    let rocket = api::build_rocket(config).manage(db);
    let client = RocketClient::tracked(rocket).await.expect("유효한 Rocket 인스턴스");
    
    // 잘못된 ID 형식으로 조회
    let response = client
        .get("/api/v1/memos/invalid-id")
        .dispatch()
        .await;
    
    assert_eq!(response.status(), Status::BadRequest);
    
    let body: Value = serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
    assert_eq!(body["error"]["code"], "BAD_REQUEST");
}
