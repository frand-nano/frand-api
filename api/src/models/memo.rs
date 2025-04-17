use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds_option;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;


// MongoDB 문서 및 API 응답에 사용될 Memo 구조체
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Memo {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "objectid_string_serializer::serialize"
    )]
    pub id: Option<ObjectId>, // DB에서 자동 생성되는 ID
    #[serde(default)] // 기본값 사용 (빈 문자열)
    pub title: String,
    #[serde(default)] // 기본값 사용 (빈 문자열)
    pub content: String,
    #[serde(with = "ts_milliseconds_option", default)] // BSON 타임스탬프 (밀리초) - 숫자 형식으로 직렬화
    pub created_at: Option<DateTime<Utc>>, // 서버에서 자동 설정
    #[serde(with = "ts_milliseconds_option", default)] // BSON 타임스탬프 (밀리초) - 숫자 형식으로 직렬화
    pub updated_at: Option<DateTime<Utc>>, // 서버에서 자동 설정
}

// 메모 생성 요청 시 사용될 DTO (Data Transfer Object)
#[derive(Deserialize, Debug, Validate)]
pub struct CreateMemoRequest {
    #[validate(length(min = 1, max = 140))]
    pub title: String,
    #[validate(length(max = 1400))]
    #[serde(default)] // 내용 필드는 선택 사항 (기본값: 빈 문자열)
    pub content: String,
}

// 메모 수정 요청 시 사용될 DTO
#[derive(Deserialize, Debug, Validate)]
pub struct UpdateMemoRequest {
    #[validate(length(min = 1, max = 140))]
    pub title: String,
    #[validate(length(max = 1400))]
    #[serde(default)] // 내용 필드는 선택 사항 (기본값: 빈 문자열)
    pub content: String,
}

// ObjectId를 16진수 문자열로 직렬화하는 모듈
mod objectid_string_serializer {
    use super::*;
    
    pub fn serialize<S>(oid: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match oid {
            Some(ref oid) => serializer.serialize_some(&oid.to_hex()), // ObjectId -> 16진수 문자열
            None => serializer.serialize_none(),
        }
    }
}
