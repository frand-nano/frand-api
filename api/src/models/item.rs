use serde::{Serialize, Deserialize};
use bson::DateTime;
use bson::oid::ObjectId;

/// 아이템 모델
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    /// MongoDB ObjectId
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    /// 아이템 이름
    pub name: String,
    
    /// 아이템 설명 (선택 사항)
    pub description: Option<String>,
    
    /// 생성 시간
    pub created_at: DateTime,
    
    /// 수정 시간
    pub updated_at: DateTime,
}

/// 아이템 수정 페이로드
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItemPayload {
    /// 이름 (수정 시 선택 사항)
    pub name: Option<String>,
    
    /// 설명 (수정 시 선택 사항)
    pub description: Option<String>,
}

/// 아이템 생성 페이로드
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemPayload {
    /// 아이템 이름
    pub name: String,
    
    /// 아이템 설명 (선택 사항)
    pub description: Option<String>,
}
