use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// DB에서 조회되거나 API 응답으로 사용될 아이템 구조체
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DBItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>,
    pub title: String,
    pub message: String,
    // 백엔드 전용 필드가 있다면 #[cfg(feature = "backend")] 등으로 분리 가능
}

// DB에서 조회되거나 API 응답으로 사용될 아이템 구조체
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    pub title: String,
    pub message: String,
    // 백엔드 전용 필드가 있다면 #[cfg(feature = "backend")] 등으로 분리 가능
}

// 아이템 생성 또는 수정을 위한 데이터 구조체
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemData {
    pub title: String,
    pub message: String,
}

impl From<DBItem> for Item {
    fn from(item: DBItem) -> Self {
        Item {
            id: item.id.map(|id| id.to_string()),
            title: item.title,
            message: item.message,
        }
    }
}