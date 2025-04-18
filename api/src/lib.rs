#[macro_use]
extern crate rocket;

use config::ApiEnvConfig;

pub mod config;

mod routes;

pub fn build_rocket(config: ApiEnvConfig) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(&config.rocket_api_endpoint, routes::routes())
        .manage(config)
}
