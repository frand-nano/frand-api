use crate::models::{Item, ItemData};
use gloo_net::http::Request;
use gloo_net::Error;

// 리버스 프록시 사용 가정
const API_ROOT: &str = "/api";

/// 모든 아이템 목록 조회
pub async fn get_items() -> Result<Vec<Item>, Error> {
    Request::get(&format!("{}/items", API_ROOT))
        .send()
        .await?
        .json::<Vec<Item>>()
        .await
}

/// 새 아이템 생성
pub async fn create_item(data: &ItemData) -> Result<Item, Error> {
    Request::post(&format!("{}/items", API_ROOT))
        .json(data)?
        .send()
        .await?
        .json::<Item>()
        .await
}

/// 아이템 수정
pub async fn update_item(item_id: &str, data: &ItemData) -> Result<Item, Error> {
    Request::put(&format!("{}/items/{}", API_ROOT, item_id))
        .json(data)?
        .send()
        .await?
        .json::<Item>()
        .await
}

/// 아이템 삭제
pub async fn delete_item(item_id: &str) -> Result<(), Error> {
    Request::delete(&format!("{}/items/{}", API_ROOT, item_id))
        .send()
        .await?;
    Ok(())
}
