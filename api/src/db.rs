use crate::models::memo::{CreateMemoRequest, Memo, UpdateMemoRequest};
use chrono::Utc;
use futures::stream::TryStreamExt;
use log::error;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection, Database,
};

const MEMO_COLLECTION: &str = "memos";

// 컬렉션 핸들 가져오기
pub fn get_memo_collection(db: &Database) -> Collection<Memo> {
    db.collection::<Memo>(MEMO_COLLECTION)
}

// 새 메모 생성
pub async fn create_memo_db(db: &Database, req: CreateMemoRequest) -> Result<Memo, mongodb::error::Error> {
    let now = Utc::now();
    
    let memo = Memo {
        id: None, // MongoDB가 ID 자동 생성
        title: req.title,
        content: req.content,
        created_at: Some(now),
        updated_at: Some(now),
    };
    
    let collection = get_memo_collection(db);
    let insert_result = collection.insert_one(memo, None).await?;
    
    // 생성된 ID로 문서 조회하여 반환
    let id = insert_result.inserted_id.as_object_id().unwrap();
    let filter = doc! { "_id": id };
    match collection.find_one(filter, None).await? {
        Some(inserted_memo) => Ok(inserted_memo),
        None => {
            error!("문서 생성 후 조회 실패: {}", id);
            Err(mongodb::error::Error::custom("문서 생성 후 조회 실패"))
        }
    }
}

// 모든 메모 조회
pub async fn list_memos_db(db: &Database) -> Result<Vec<Memo>, mongodb::error::Error> {
    let collection = get_memo_collection(db);
    let filter = doc! {}; // 빈 필터로 모든 문서 조회
    
    let cursor = collection.find(filter, None).await?;
    let result = cursor.try_collect().await?;
    
    Ok(result)
}

// ID로 특정 메모 조회
pub async fn get_memo_db(db: &Database, id: ObjectId) -> Result<Option<Memo>, mongodb::error::Error> {
    let collection = get_memo_collection(db);
    let filter = doc! { "_id": id };
    collection.find_one(filter, None).await
}

// 특정 메모 수정
pub async fn update_memo_db(
    db: &Database,
    id: ObjectId,
    req: UpdateMemoRequest,
) -> Result<Option<Memo>, mongodb::error::Error> {
    let collection = get_memo_collection(db);
    let filter = doc! { "_id": id };
    
    // 현재 시간을 밀리초 타임스탬프로 변환
    let now = Utc::now();
    let timestamp_millis = now.timestamp_millis();
    
    let update = doc! {
        "$set": {
            "title": req.title,
            "content": req.content,
            "updated_at": timestamp_millis
        }
    };
    
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    
    collection.find_one_and_update(filter, update, options).await
}

// 특정 메모 삭제
pub async fn delete_memo_db(
    db: &Database,
    id: ObjectId,
) -> Result<bool, mongodb::error::Error> {
    let collection = get_memo_collection(db);
    let filter = doc! { "_id": id };
    
    let result = collection.delete_one(filter, None).await?;
    Ok(result.deleted_count > 0)
}
