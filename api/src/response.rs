use serde::Serialize;
use rocket::http::Status;
use serde_json::Value;

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
    pub fn new(data: T) -> Self {
        ApiResponse {
            success: true,
            data,
        }
    }
}

// 오류 응답을 위한 표준 형식
#[derive(Serialize)]
pub struct ApiError {
    pub success: bool,
    pub error: ErrorDetails,
    #[serde(skip_serializing)]
    pub status: Status,
}

#[derive(Serialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>, status: Status) -> Self {
        ApiError {
            success: false,
            error: ErrorDetails {
                code: code.into(),
                message: message.into(),
                details: None,
            },
            status,
        }
    }

    pub fn with_details(self, details: Value) -> Self {
        ApiError {
            error: ErrorDetails {
                details: Some(details),
                ..self.error
            },
            ..self
        }
    }
}
