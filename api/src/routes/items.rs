use rocket::{Route, routes, get, post, put, delete, State};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use mongodb::Database;
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use bson::DateTime;
use log::error;
use crate::models::item::{Item, UpdateItemPayload, CreateItemPayload};

/// 아이템 라우트 정의
pub fn routes() -> Vec<Route> {
    routes![create_item, get_all_items, get_item_by_id, update_item, delete_item]
}

/// 새 아이템 생성
#[post("/", data = "<payload>")]
pub async fn create_item(state: &State<Database>, payload: Json<CreateItemPayload>) -> Result<status::Created<Json<Item>>, Status> {
    let db = &**state;
    let collection = db.collection::<Item>("items");
    
    // CreateItemPayload에서 Item 생성
    let new_item = Item {
        id: None,
        name: payload.name.clone(),
        description: payload.description.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };
    
    // MongoDB에 저장
    let result = collection.insert_one(&new_item, None).await;
    match result {
        Ok(insert_result) => {
            // 생성된 ID 가져오기
            let id = insert_result.inserted_id.as_object_id()
                .ok_or_else(|| {
                    error!("MongoDB가 유효한 ObjectId를 반환하지 않았습니다");
                    Status::InternalServerError
                })?;
            
            // 생성된 아이템 조회
            collection.find_one(doc! { "_id": id }, None).await
                .map_err(|e| {
                    error!("아이템 조회 중 오류 발생: {}", e);
                    Status::InternalServerError
                })?
                .ok_or_else(|| {
                    error!("방금 생성된 아이템을 찾을 수 없습니다");
                    Status::InternalServerError
                })
                .map(|created_item| {
                    status::Created::new("").body(Json(created_item))
                })
        }
        Err(e) => {
            error!("아이템 삽입 중 오류 발생: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// 모든 아이템 조회
#[get("/")]
pub async fn get_all_items(state: &State<Database>) -> Result<Json<Vec<Item>>, Status> {
    let db = &**state;
    let collection = db.collection::<Item>("items");
    
    // 모든 문서 조회
    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    // 결과를 Vec으로 수집
    let mut items = Vec::new();
    while let Some(result) = cursor.try_next().await.map_err(|_| Status::InternalServerError)? {
        items.push(result);
    }
    
    Ok(Json(items))
}

/// 특정 ID로 아이템 조회
#[get("/<id>")]
pub async fn get_item_by_id(state: &State<Database>, id: &str) -> Result<Json<Item>, Status> {
    let db = &**state;
    let collection = db.collection::<Item>("items");
    
    // ID 파싱
    let object_id = match ObjectId::parse_str(id) {
        Ok(oid) => oid,
        Err(_) => return Err(Status::BadRequest),
    };
    
    // ID로 아이템 조회
    match collection.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(item)) => Ok(Json(item)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// 아이템 업데이트
#[put("/<id>", data = "<payload>")]
pub async fn update_item(
    state: &State<Database>, 
    id: &str, 
    payload: Json<UpdateItemPayload>
) -> Result<Json<Item>, Status> {
    let db = &**state;
    let collection = db.collection::<Item>("items");
    
    // ID 파싱
    let object_id = match ObjectId::parse_str(id) {
        Ok(oid) => oid,
        Err(_) => return Err(Status::BadRequest),
    };
    
    // 업데이트할 필드 준비
    let mut update_doc = doc! {};
    
    if let Some(name) = &payload.name {
        update_doc.insert("name", name);
    }
    
    if let Some(description) = &payload.description {
        update_doc.insert("description", description);
    }
    
    // 수정 시간 업데이트
    update_doc.insert("updatedAt", DateTime::now());
    
    // 아이템 업데이트
    let update_result = collection
        .update_one(
            doc! { "_id": object_id },
            doc! { "$set": update_doc },
            None
        )
        .await;
    
    match update_result {
        Ok(result) => {
            if result.matched_count == 0 {
                return Err(Status::NotFound);
            }
            
            // 업데이트된 아이템 조회 후 반환
            match collection.find_one(doc! { "_id": object_id }, None).await {
                Ok(Some(item)) => Ok(Json(item)),
                Ok(None) => Err(Status::NotFound),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

/// 아이템 삭제
#[delete("/<id>")]
pub async fn delete_item(state: &State<Database>, id: &str) -> Result<Status, Status> {
    let db = &**state;
    let collection = db.collection::<Item>("items");
    
    // ID 파싱
    let object_id = match ObjectId::parse_str(id) {
        Ok(oid) => oid,
        Err(_) => return Err(Status::BadRequest),
    };
    
    // 아이템 삭제
    match collection.delete_one(doc! { "_id": object_id }, None).await {
        Ok(result) => {
            if result.deleted_count == 0 {
                return Err(Status::NotFound);
            }
            Ok(Status::NoContent)
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
