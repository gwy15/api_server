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

#[test]
fn test_login() {
    use crate::account::User;
    use crate::test::prelude::*;
    let rocket = test_rocket();
    let conn = PgConn::get_one(&rocket).unwrap();
    let client = Client::new(rocket).unwrap();
    // prepare: insert user
    let user = User::new("test_account".into(), "password".into(), &conn).unwrap();
    let request = || client.post("/login").header(ContentType::Form);

    // ok
    let req = request().body("username=test_account&password=password");
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Ok);
    // wrong password
    let req = request().body("username=test_account&password=wrong_password");
    let mut resp = req.dispatch();
    assert_eq!(resp.status(), Status::Unauthorized);
    let errmsg: ErrMsg = from_str(&resp.body_string().unwrap()).unwrap();
    assert!(errmsg.errmsg.contains("not found"));
    assert!(errmsg.detail.is_none());
    // user not exist
    let req = request().body("username=test_account2&password=password");
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Unauthorized);

    // make user banned
    diesel::update(&user)
        .set(schema::users::is_disabled.eq(true))
        .execute(&*conn)
        .unwrap();
    let req = request().body("username=test_account&password=password");
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Unauthorized);
}
