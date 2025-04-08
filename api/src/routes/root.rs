use rocket::{Route, get, routes};

#[get("/")]
fn hello() -> &'static str {
    "hello world"
}

pub fn routes() -> Vec<Route> {
    routes![hello]
}
