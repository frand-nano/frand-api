use rocket::local::blocking::Client;
use api::config::ApiEnvConfig;

pub fn new_test_env_config() -> anyhow::Result<ApiEnvConfig> {
    Ok(ApiEnvConfig::load("../.env")?)
}

pub fn new_test_rocket_client() -> anyhow::Result<Client> {
    let config = new_test_env_config()?;    
    let rocket = api::build_rocket(config);
    
    Ok(Client::tracked(rocket)?)
}
