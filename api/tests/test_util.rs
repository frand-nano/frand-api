use rocket::local::blocking::Client;
use api::config::ApiConfig;

pub fn new_test_rocket_client() -> anyhow::Result<Client> {
    let config = ApiConfig::load("../config/test.toml")?;    
    let rocket = api::build_rocket(config);
    
    Ok(Client::tracked(rocket)?)
}
