use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: u16,
    version: &'static str,
}

#[get("/health")]
pub fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: 200,
        version: env!("CARGO_PKG_VERSION"),
    })
}
