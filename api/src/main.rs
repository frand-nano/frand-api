use api::config::ApiEnvConfig;

#[rocket::launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    println!("⚙️  frand API 서비스 시작 중...");

    let config = ApiEnvConfig::load(".env").unwrap();

    simple_logger::init_with_level(config.log_level()).unwrap();        
    
    api::build_rocket(config)
}