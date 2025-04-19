#[macro_use]
extern crate rocket;

use config::ApiEnvConfig;

pub mod config;

mod routes;
mod error;
mod models;
mod mongodb;

pub fn build_rocket(config: ApiEnvConfig) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(config.rocket_api_endpoint(), routes::routes())
        .manage(config)
        .attach(mongodb::MongoDBFairing)
}
