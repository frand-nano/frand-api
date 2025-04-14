use rocket::get;
use rocket::serde::json::Json;

use crate::models::health::HealthStatus;
use crate::response::ApiResponse;

// 헬스체크 경로 핸들러
#[get("/health")]
pub async fn health_check_handler() -> Json<ApiResponse<HealthStatus>> {
    let health_status = HealthStatus::ok();
    let api_response = ApiResponse::new(health_status);
    
    Json(api_response)
}
