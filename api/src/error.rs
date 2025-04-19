use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("리소스가 존재하지 않습니다: {0}")]
    NotFound(String),

    #[error("잘못된 요청입니다: {0}")]
    BadRequest(String),

    #[error("데이터베이스 ObjectId 생성 오류")]
    BsonOidError(#[from] mongodb::bson::oid::Error),

    #[error("데이터베이스 오류")]
    MongoDBError(#[from] mongodb::error::Error),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let status = match &self {
            ApiError::NotFound(err) => {
                info!("{self}: {err} \n 요청: {req}");
                Status::NotFound
            },
            ApiError::BadRequest(err) => {
                info!("{self}: {err} \n 요청: {req}");
                Status::BadRequest
            },
            ApiError::BsonOidError(err) => {
                info!("{self}: {err} \n 요청: {req}");
                Status::BadRequest
            },
            ApiError::MongoDBError(err) => {
                error!("{self}: {err} \n 요청: {req}");
                Status::InternalServerError
            },
        };
        
        let error_response = ErrorResponse {
            error: status.reason().unwrap_or("오류 발생").to_string(),
            message: self.to_string(),
        };
        
        let json_string = serde_json::to_string(&error_response)
            .map_err(|err| {
                error!("JSON 직렬화 오류: {err}");
                Status::InternalServerError
            })?;

        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_string.len(), std::io::Cursor::new(json_string))
            .ok()
    }
}