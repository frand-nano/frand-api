use rocket::http::Status;
use test_util::new_test_rocket_client;

mod test_util;

#[test]
fn test_health() -> anyhow::Result<()> {
    let client = new_test_rocket_client()?;
    let response = client.get("/api/v1/health").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    
    let body: serde_json::Value = response.into_json().expect("유효한 JSON 응답이 아닙니다");

    assert_eq!(body["status"], 200);
    assert!(body.get("version").is_some(), "버전 정보가 누락되었습니다");

    Ok(())
}