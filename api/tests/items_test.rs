mod test_common;

use test_common::load_env_test;
use rocket::http::{Status, ContentType};
use crate::test_common::{setup_test_db, setup_test_client};
use api::models::DBItem;
use bson::oid::ObjectId;

/// 테스트 시작 시 실행 (일회성)
#[tokio::test]
async fn test_items_crud() {
    // 테스트 환경 설정
    load_env_test();

    // 테스트 DB 설정
    let db = setup_test_db().await;
    
    // Rocket 인스턴스 생성
    let client = setup_test_client(db.clone()).await;
    
    // 1. 아이템 생성 테스트
    let new_item = r#"{
        "title": "테스트 아이템",
        "message": "테스트 설명입니다"
    }"#;
    
    let response = client
        .post("/items")
        .header(ContentType::JSON)
        .body(new_item)
        .dispatch().await;
    
    assert_eq!(response.status(), Status::Created);
    let created_item: DBItem = serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
    assert!(created_item.id.is_some());
    assert_eq!(created_item.title, "테스트 아이템");
    assert_eq!(created_item.message, "테스트 설명입니다".to_string());
    
    let item_id = created_item.id.unwrap().to_string();
    
    // 2. 모든 아이템 조회 테스트
    let response = client.get("/items").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let items: Vec<DBItem> = serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
    assert_eq!(items.len(), 1);
    
    // 3. 특정 ID로 아이템 조회 테스트
    let response = client.get(format!("/items/{}", item_id)).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let item: DBItem = serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
    assert_eq!(item.title, "테스트 아이템");
    
    // 4. 아이템 업데이트 테스트
    let update_payload = r#"{
        "title": "수정된 아이템",
        "message": ""
    }"#;
    
    let response = client
        .put(format!("/items/{}", item_id))
        .header(ContentType::JSON)
        .body(update_payload)
        .dispatch().await;
    
    assert_eq!(response.status(), Status::Ok);
    let updated_item: DBItem = serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
    assert_eq!(updated_item.title, "수정된 아이템");
    assert_eq!(updated_item.message, "".to_string());
    
    // 5. 아이템 삭제 테스트
    let response = client.delete(format!("/items/{}", item_id)).dispatch().await;
    assert_eq!(response.status(), Status::NoContent);
    
    // 삭제 후 조회 시 404 확인
    let response = client.get(format!("/items/{}", item_id)).dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
    
    // 6. 존재하지 않는 ID 조회 테스트
    let non_existent_id = ObjectId::new().to_string();
    let response = client.get(format!("/items/{}", non_existent_id)).dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
    
    // 7. 잘못된 ID 형식 테스트
    let response = client.get("/items/invalid-id").dispatch().await;
    assert_eq!(response.status(), Status::BadRequest);
}
