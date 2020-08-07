use crate::{
    account::{Error as AccountError, JWT},
    Config, Error, PgConn,
};
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

        // error: if missing token, return 401 Unauthorized
        let token = option_to_outcome! {
            request.headers().get("Authorization").next()
            => (Unauthorized, AccountError::NoLoginToken)
        };

        // verify JWT token
        let config = request.guard::<State<Config>>().expect("Config not found.");
        let secret = &config.jwt_secret;
        // error: jwt verification failed
        let jwt = result_to_outcome! {
            JWT::from_token(token, &secret) => (Unauthorized, AccountError::BadLoginToken)
        };

        // get user
        let user_id = jwt.user_id();
        let con = request
            .guard::<PgConn>()
            .expect("Failed to get DB connection");
        let user_result = users
            .filter(id.eq(user_id))
            .get_result::<User>(&*con)
            .optional();

        // error: DB query failed
        let user = result_to_outcome! { user_result => (Unauthorized, Error::Database) };
        // error: no such user
        let user = option_to_outcome! { user => (Unauthorized, AccountError::UserNotFound) };
        // error: user token expired
        if jwt.issued_at() < user.token_valid_after.timestamp() {
            return Outcome::Failure((Status::Unauthorized, AccountError::TokenExpired.into()));
        }
        // error: user banned
        if user.is_disabled {
            return Outcome::Failure((Status::Unauthorized, AccountError::Banned.into()));
        }

        Outcome::Success(user)
    }
}
