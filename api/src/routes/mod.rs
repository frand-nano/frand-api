mod health;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        health::health,
    ]
}