use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::{futures::TryStreamExt, serde::json::Json};
use rocket::State;
use frand_api_common::models::user::{CreateUserDto, UserDto};
use crate::models::User;
use crate::{error::ApiError, mongodb::MongoDB};

#[get("/users")]
pub async fn get_users(db: &State<MongoDB>) -> Result<Json<Vec<UserDto>>, ApiError> {
    let users: Vec<User> =db.users()
        .find(doc!{}).await?
        .try_collect::<Vec<User>>().await?;

    let users: Vec<UserDto> = users.into_iter()
        .map(|user| user.into())
        .collect(); 

    Ok(Json(users))
}

#[post("/users", format = "json", data = "<user>")]
pub async fn create_user(user: Json<CreateUserDto>, db: &State<MongoDB>) -> Result<Json<UserDto>, ApiError> {
    let users = db.users();

    let exists = users.find_one(doc! { "google_id": &user.google_id }).await?;
    
    if exists.is_some() {
        return Err(ApiError::BadRequest(format!("이미 존재하는 사용자입니다: {}", user.email)));
    }

    let new_user: User = user.into_inner().into();
    let user_id = users.insert_one(&new_user).await?.inserted_id.as_object_id().unwrap();
    let created_user = db.try_get_user(user_id).await?.into();    

    Ok(Json(created_user))
}

#[get("/users/<user_id>")]
pub async fn get_user(user_id: &str, db: &State<MongoDB>) -> Result<Json<UserDto>, ApiError> {
    let user_id = ObjectId::parse_str(user_id)?;
    let user = db.try_get_user(user_id).await?.into();

    Ok(Json(user))
}

#[delete("/users/<user_id>")]
pub async fn delete_user(user_id: &str, db: &State<MongoDB>) -> Result<Status, ApiError> {    
    let user_id = ObjectId::parse_str(user_id)?;
    db.users().delete_one(doc! { "_id": user_id }).await?;

    Ok(Status::NoContent)
}