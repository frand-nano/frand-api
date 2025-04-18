use api::config::ApiConfig;

#[rocket::launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    println!("⚙️  frand API 서비스 시작 중...");

    let config = ApiConfig::load("config/default.toml").unwrap();

    simple_logger::init_with_level(config.log_level()).unwrap();        
    
    api::build_rocket(config)
}