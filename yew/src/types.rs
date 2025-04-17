use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// API 응답 래퍼 타입
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[allow(dead_code)]
    pub success: bool,
    pub data: T,
}

// API 오류 응답 타입
#[derive(Debug, Deserialize)]
pub struct ApiError {
    #[allow(dead_code)]
    pub success: bool,
    #[allow(dead_code)]
    pub error: ErrorDetails,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetails {
    #[allow(dead_code)]
    pub code: String,
    #[allow(dead_code)]
    pub message: String,
    #[allow(dead_code)]
    pub details: Option<serde_json::Value>,
}

// API에서 받는 메모 데이터 구조체
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MemoFrontend {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    #[serde(with = "chrono::serde::ts_milliseconds_option", default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_milliseconds_option", default)]
    pub updated_at: Option<DateTime<Utc>>,
}

// 폼 입력용 메모 데이터 구조체
#[derive(Debug, Clone, Default, Serialize, Validate, PartialEq)]
pub struct MemoData {
    #[validate(length(min = 1, max = 140, message = "제목은 1~140자 사이여야 합니다."))]
    pub title: String,
    
    #[validate(length(max = 1400, message = "내용은 1400자를 초과할 수 없습니다."))]
    pub content: String,
}

impl From<&MemoFrontend> for MemoData {
    fn from(memo: &MemoFrontend) -> Self {
        Self {
            title: memo.title.clone(),
            content: memo.content.clone(),
        }
    }
}
