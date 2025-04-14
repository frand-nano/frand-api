pub mod config;
pub mod error;
pub mod handlers;
pub mod logger;
pub mod models;
pub mod response;

use config::Config;
use rocket::{self, Build, Rocket};

// Rocket 인스턴스 구성 및 생성
pub fn build_rocket(config: Config) -> Rocket<Build> {
    let figment = rocket::Config::figment()
        .merge(("address", config.rocket_address))
        .merge(("port", config.rocket_port));

    rocket::custom(figment)
        .mount("/", rocket::routes![
            handlers::root::root_handler,
        ])
        .mount("/api/v1", rocket::routes![
            handlers::health::health_check_handler,
        ])
        .register("/", rocket::catchers![
            error::not_found,
            error::method_not_allowed,
            error::internal_error,
        ])
}
