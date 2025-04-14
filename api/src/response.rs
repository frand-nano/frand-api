use serde::Serialize;

// 성공 응답을 위한 표준 형식
#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    // 새로운 성공 응답 생성
    pub fn new(data: T) -> Self {
        ApiResponse {
            success: true,
            data,
        }
    }
}
