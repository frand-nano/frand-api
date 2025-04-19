use mongodb::{bson::{doc, oid::ObjectId}, Client, Database};
use rocket::{fairing::{self, Fairing}, Build, Rocket};
use crate::{config::ApiEnvConfig, error::ApiError, models::User};

pub struct MongoDB {
    database: Database,
}

pub struct MongoDBFairing;

impl MongoDB {
    pub async fn new(config: &ApiEnvConfig) -> Self {
        let client = Client::with_uri_str(config.mongo_uri());
        let database = client.await.unwrap().database(config.mongo_db_name());
        MongoDB { database }
    }

    pub fn users(&self) -> mongodb::Collection<User> {
        self.database.collection::<User>("users")
    }

    pub async fn try_get_user(&self, id: ObjectId) -> Result<User, ApiError> {
        self.users().find_one(doc! { "_id": id }).await?
        .ok_or(ApiError::NotFound("존재하지 않는 유저입니다.".to_string()))
    }
}

#[rocket::async_trait]
impl Fairing for MongoDBFairing {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "MongoDB Connection",
            kind: fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let config = match rocket.state::<ApiEnvConfig>() {
            Some(config) => config,
            None => {
                error!("ApiEnvConfig를 찾을 수 없습니다. Rocket에 설정이 등록되었는지 확인하세요.");
                return Err(rocket);
            }
        };

        let mongo_db = MongoDB::new(config).await;

        Ok(rocket.manage(mongo_db))
    }
}