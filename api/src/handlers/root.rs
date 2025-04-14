use rocket::get;

// 루트 경로 핸들러
#[get("/")]
pub async fn root_handler() -> &'static str {
    "hello world"
}
