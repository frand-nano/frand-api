mod health;
mod users;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        health::health,
        users::get_users,
        users::create_user,
        users::get_user,
        users::delete_user,
    ]
}