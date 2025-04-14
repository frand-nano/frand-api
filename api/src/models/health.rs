use serde::Serialize;

// 헬스체크 응답 구조체
#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
}

impl HealthStatus {
    // 정상 상태 응답 생성
    pub fn ok() -> Self {
        HealthStatus {
            status: "ok".to_string(),
        }
    }
}
