mod jwt;
pub use jwt::JWT;

mod models;
pub use models::{NewUser, User};

mod errors;
pub use errors::Error;

mod routes;

pub fn routes() -> Vec<rocket::Route> {
    routes![routes::login, routes::register,]
}
