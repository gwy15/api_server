use super::Error as AccountError;
use crate::{schema::users, Config, Error, Form, PgConn, Result, User};
use chrono::Utc;
use diesel::prelude::*;
use rocket::State;
use rocket_contrib::json::Json;

pub mod helpers {
    /// The login/register form
    #[derive(FromForm)]
    pub struct LoginInfo {
        pub username: String,
        pub password: String,
    }

    /// The returned structure on success login or register
    #[derive(Debug, Serialize)]
    pub struct TokenInfo {
        pub token: String,
    }
}
use helpers::{LoginInfo, TokenInfo};

/// Login user
#[post("/login", data = "<form>")]
pub fn login(
    form: Form<LoginInfo>,
    conn: PgConn,
    config: State<Config>,
) -> Result<Json<TokenInfo>> {
    let form = form?.into_inner();
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

/// Register new user
#[post("/register", data = "<form>")]
pub fn register(
    form: Form<LoginInfo>,
    conn: PgConn,
    config: State<Config>,
) -> Result<Json<TokenInfo>> {
    let form = form?.into_inner();
    // verify username and password complexity
    use zxcvbn::zxcvbn;
    let entropy =
        zxcvbn(&form.password, &[&form.username]).map_err(|_| AccountError::PasswordTooWeak)?;
    if entropy.score() < 3 {
        return Err(Error::Authorization(AccountError::PasswordTooWeak));
    }

    //
    let user = User::new(form.username, form.password, &conn)?;
    //
    let token = user.generate_token(config.jwt_duration, &config.jwt_secret);
    // TODO: return user info
    let _user: User = diesel::update(&user)
        .set(users::last_login.eq(Utc::now()))
        .get_result(&*conn)?;

    Ok(Json(TokenInfo { token }))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_login() {
        use crate::test::prelude::*;
        let (client, conn) = setup();
        // prepare: insert user
        let user = User::new("test_account".into(), "password".into(), &conn).unwrap();
        let request = |s| {
            client
                .post("/account/login")
                .header(ContentType::Form)
                .body(s)
                .dispatch()
        };

        // ok
        let resp = request("username=test_account&password=password");
        assert_eq!(resp.status(), Status::Ok);
        // wrong password
        let resp = request("username=test_account&password=wrong_password");
        assert_eq!(resp.status(), Status::Unauthorized);
        let errmsg = errmsg_from(resp);
        assert!(errmsg.errmsg.contains("not found"));
        assert!(errmsg.detail.is_none());
        // user not exist
        let resp = request("username=test_account2&password=password");
        assert_eq!(resp.status(), Status::Unauthorized);

        // make user banned
        diesel::update(&user)
            .set(schema::users::is_disabled.eq(true))
            .execute(&*conn)
            .unwrap();
        let resp = request("username=test_account&password=password");
        assert_eq!(resp.status(), Status::Forbidden);
    }

    #[test]
    fn test_register() {
        use crate::test::prelude::*;
        let (client, _) = setup();
        //
        let request = |s| {
            client
                .post("/account/register")
                .header(ContentType::Form)
                .body(s)
                .dispatch()
        };
        // normal register
        let resp = request("username=test_account&password=longC0mplex_p5wd");
        assert_eq!(resp.status(), Status::Ok);
        // username occupied
        let resp = request("username=test_account&password=longC0mplex_p5wd");
        assert_eq!(resp.status(), Status::BadRequest);
        let errmsg = errmsg_from(resp);
        assert_eq!(errmsg.errmsg, r#"Username "test_account" occupied"#);
        // weak password
        for pswd in &["", "pswd", "123", "test_account_p5wd"] {
            let body = format!("username=test_account&password={}", pswd);
            let resp = client
                .post("/account/register")
                .header(ContentType::Form)
                .body(&body)
                .dispatch();
            assert_eq!(resp.status(), Status::BadRequest);
            let errmsg = errmsg_from(resp);
            assert_eq!(errmsg.errmsg, r#"Password too weak"#);
        }
    }
}
