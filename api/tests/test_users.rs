use frand_api::config::ApiEnvConfig;
use mongodb::bson::Uuid;
use rocket::{http::{ContentType, Status}, local::blocking::Client};
use test_util::{new_test_env_config, new_test_rocket_client};
use frand_api_common::models::user::UserDto;
use serde_json::json;
use serial_test::serial;

mod test_util;

fn generate_unique_suffix() -> String {
    let uuid = Uuid::new();
    uuid.to_string().split('-').next().unwrap_or("test").to_string()
}

fn create_user(client: &Client, config: &ApiEnvConfig, suffix: &str) -> anyhow::Result<UserDto> {
    let user_data = json!({
        "google_id": format!("google_{}", suffix),
        "email": format!("test_{}@example.com", suffix),
        "username": format!("사용자_{}", suffix)
    });

    let response = client
        .post(format!("{}/users", config.rocket_api_endpoint()))
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let created_user: UserDto = response.into_json().expect("유효한 JSON 응답이 아닙니다");
    assert!(!created_user.id.is_empty());
    assert_eq!(created_user.google_id, format!("google_{}", suffix));
    assert_eq!(created_user.email, format!("test_{}@example.com", suffix));
    assert_eq!(created_user.username, format!("사용자_{}", suffix));

    Ok(created_user)
}

fn cleanup_test_user(client: &Client, config: &ApiEnvConfig, user_id: &str) -> anyhow::Result<()> {
    client.delete(format!("{}/users/{}", config.rocket_api_endpoint(), user_id)).dispatch();

    Ok(())
}

#[test]
#[serial]
fn test_create_user() -> anyhow::Result<()> {
    let client = new_test_rocket_client()?;
    let config = new_test_env_config()?;
    let suffix = generate_unique_suffix();
    let user = create_user(&client, &config, &suffix)?;

    cleanup_test_user(&client, &config, &user.id)?;

    Ok(())
}


#[test]
#[serial]
fn test_get_user() -> anyhow::Result<()> {
    let client = new_test_rocket_client()?;
    let config = new_test_env_config()?;
    let suffix = generate_unique_suffix();
    let user_id = create_user(&client, &config, &suffix)?.id;

    let response = client
        .get(format!("{}/users/{}", config.rocket_api_endpoint(), user_id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    
    let user: UserDto = response.into_json().expect("유효한 JSON 응답이 아닙니다");
    assert_eq!(user.id, user_id);
    assert_eq!(user.google_id, format!("google_{}", suffix));
    assert_eq!(user.email, format!("test_{}@example.com", suffix));
    
    cleanup_test_user(&client, &config, &user_id)?;

    let response = client
        .get(format!("{}/users/{}", config.rocket_api_endpoint(), "invalid_id"))
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);

    let response = client
        .get(format!("{}/users/{}", config.rocket_api_endpoint(), user_id))
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);

    Ok(())
}

#[test]
#[serial]
fn test_delete_user() -> anyhow::Result<()> {
    let client = new_test_rocket_client()?;
    let config = new_test_env_config()?;
    let suffix = generate_unique_suffix();
    let user_id = create_user(&client, &config, &suffix)?.id;

    let response = client.delete(format!("{}/users/{}", config.rocket_api_endpoint(), user_id)).dispatch();
    
    assert_eq!(response.status(), Status::NoContent);
    
    let response = client
        .get(format!("{}/users/{}", config.rocket_api_endpoint(), user_id))
        .dispatch();
    
    assert_eq!(response.status(), Status::NotFound);
    
    Ok(())
}