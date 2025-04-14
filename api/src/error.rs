use rocket::{Request, Response, response::{self, Responder}};
use rocket::http::Status;
use serde::Serialize;
use serde_json::Value;

// 오류 코드 상수
pub const BAD_REQUEST: &str = "BAD_REQUEST";
pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
pub const FORBIDDEN: &str = "FORBIDDEN";
pub const NOT_FOUND: &str = "NOT_FOUND";
pub const METHOD_NOT_ALLOWED: &str = "METHOD_NOT_ALLOWED";
pub const INTERNAL_SERVER_ERROR: &str = "INTERNAL_SERVER_ERROR";
pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";

// 오류 세부 정보를 담는 구조체
#[derive(Serialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

// API 오류 응답 구조체
#[derive(Serialize)]
pub struct ApiError {
    pub success: bool,
    pub error: ErrorDetails,
    #[serde(skip_serializing)]
    pub status: Status,
}

impl ApiError {
    // 새로운 API 오류 응답 생성
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

    // 상세 정보 추가
    pub fn with_details(self, details: Value) -> Self {
        ApiError {
            error: ErrorDetails {
                code: self.error.code,
                message: self.error.message,
                details: Some(details),
            },
            ..self
        }
    }
}

// Rocket Responder 구현
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self).unwrap();
        
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(self.status)
            .sized_body(body.len(), std::io::Cursor::new(body))
            .ok()
    }
}

// 404 Not Found Catcher
#[rocket::catch(404)]
pub fn not_found(req: &Request) -> ApiError {
    ApiError::new(
        NOT_FOUND,
        format!("경로를 찾을 수 없습니다: {}", req.uri()),
        Status::NotFound
    )
}

// 405 Method Not Allowed Catcher
#[rocket::catch(405)]
pub fn method_not_allowed(req: &Request) -> ApiError {
    ApiError::new(
        METHOD_NOT_ALLOWED,
        format!("허용되지 않은 메소드: {} {}", req.method(), req.uri()),
        Status::MethodNotAllowed
    )
}

// 500 Internal Server Error Catcher
#[rocket::catch(500)]
pub fn internal_error() -> ApiError {
    ApiError::new(
        INTERNAL_SERVER_ERROR,
        "서버 내부 오류가 발생했습니다",
        Status::InternalServerError
    )
}
