#[macro_use]
extern crate rocket;

use config::ApiConfig;

pub mod config;

mod routes;

pub fn build_rocket(config: ApiConfig) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(&config.server.api_version, routes::routes())
        .manage(config)
}
