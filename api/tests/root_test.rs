mod test_common;

use test_common::{load_env_test, setup_test_client, setup_test_db};
use rocket::http::Status;

#[tokio::test]
async fn test_root_endpoint() {
    // 테스트 환경 설정
    load_env_test();

    let db = setup_test_db().await;

    let client = setup_test_client(db).await;
    
    // 루트 경로 테스트
    let response = client.get("/").dispatch().await;
    
    // 응답 검증
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "hello world");
}
