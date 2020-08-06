use crate::{account::JWT, Config, Error, PgConn};
use chrono::prelude::*;
use diesel::prelude::*;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};

type DT = DateTime<Utc>;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub is_disabled: bool,
    pub last_login: DT,
    pub token_valid_after: DT,
    pub created_at: DT,
    pub updated_at: DT,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = Error;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        use crate::schema::users::dsl::*;

        log::debug!("Parsing user...");

        // if missing token, return 401 Unauthorized
        let token_result = request.headers().get("Authorization").next();
        let token = match token_result {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, Error::NoLoginToken)),
        };

        let config = request.guard::<State<Config>>().expect("Config not found.");
        let secret = &config.jwt_secret;

        // parse token
        let jwt = match JWT::from_token(token, &secret) {
            Ok(jwt) => jwt,
            Err(e) => return Outcome::Failure((Status::Unauthorized, Error::BadLoginToken(e))),
        };
        let user_id = jwt.user_id();

        let con = request
            .guard::<PgConn>()
            .expect("Failed to get DB connection");

        let user_result = users
            .filter(id.eq(user_id))
            .get_result::<User>(&*con)
            .optional();
        let user = match user_result {
            Ok(user) => user,
            Err(e) => return Outcome::Failure((Status::Unauthorized, Error::Database(e))),
        };
        let user = match user {
            Some(user) => user,
            None => return Outcome::Failure((Status::Unauthorized, Error::UserNotFound)),
        };

        Outcome::Success(user)
    }
}
