pub mod routes;
pub mod services;
pub mod models;
pub mod config;

use rocket::Rocket;
use rocket::build;

/// Rocket 인스턴스 생성 함수
/// 
/// 테스트 등에서 재사용 가능한 방식으로 Rocket 인스턴스를 생성합니다.
pub fn create_rocket() -> Rocket<rocket::Build> {
    build()
        .mount("/", routes::root::routes())
}
