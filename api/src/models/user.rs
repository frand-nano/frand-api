use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use frand_api_common::models::user::{CreateUserDto, UserDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    google_id: String,
    email: String,
    username: String,
    created_at: DateTime,
    updated_at: DateTime,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id.map(|id| id.to_string()).unwrap_or_default(),
            google_id: user.google_id,
            email: user.email,
            username: user.username,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}

impl From<UserDto> for User {
    fn from(user: UserDto) -> Self {
        User {
            id: ObjectId::parse_str(&user.id).ok(),
            google_id: user.google_id,
            email: user.email,
            username: user.username,
            created_at: DateTime::parse_rfc3339_str(&user.created_at).unwrap(),
            updated_at: DateTime::parse_rfc3339_str(&user.updated_at).unwrap(),
        }
    }
}

impl From<CreateUserDto> for User {
    fn from(user: CreateUserDto) -> Self {
        User {
            id: None,
            google_id: user.google_id,
            email: user.email,
            username: user.username,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
