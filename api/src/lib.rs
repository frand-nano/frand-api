pub mod routes;
pub mod services;
pub mod models;
pub mod config;

use rocket::Rocket;
use rocket::Build;
use mongodb::Database;

/// Rocket 인스턴스 생성 함수
/// 
/// MongoDB 데이터베이스 인스턴스를 받아 Rocket 상태로 관리합니다.
pub fn create_rocket(db: Database) -> Rocket<Build> {
    rocket::build()
        .manage(db)
        .mount("/", routes::root::routes())
        .mount("/items", routes::items::routes())
}
