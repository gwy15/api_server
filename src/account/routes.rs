use crate::{schema::users, Config, Form, PgConn, Result, User};
use chrono::Utc;
use diesel::prelude::*;
use rocket::State;
use rocket_contrib::json::Json;

#[derive(FromForm)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

/// The returned structure on success login or register
#[derive(Debug, Serialize)]
pub struct TokenInfo {
    token: String,
}

/// Login user
#[post("/login", data = "<form>")]
pub fn login(
    form: Form<LoginInfo>,
    conn: PgConn,
    config: State<Config>,
) -> Result<Json<TokenInfo>> {
    let form = form?;
    // try get user
    let user = User::from(&form.username, &form.password, &conn)?;
    // generate token and return
    let token = user.generate_token(config.jwt_duration, &config.jwt_secret);

    // maintain last login
    let _user: User = diesel::update(&user)
        .set(users::last_login.eq(Utc::now()))
        .get_result(&*conn)?;

    // todo: return user info

    Ok(Json(TokenInfo { token }))
}
