use crate::{
    account::{Error as AccountError, JWT},
    schema::users,
    Config, Error, PgConn, Result,
};
use chrono::prelude::*;
use diesel::prelude::*;
use rand::prelude::*;
use ring::{digest, pbkdf2};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};
use std::num::NonZeroU32;

lazy_static! {
    static ref PSWD_ROUNDS: NonZeroU32 = NonZeroU32::new(70_000).unwrap();
    static ref PSWD_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
}

const PSWD_LENGTH: usize = digest::SHA256_OUTPUT_LEN;

/// The user class for application.
#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    /// user id
    pub id: i32,
    /// username (nickname)
    pub username: String,
    /// hashed password (using bcrypt or pbkdf2_sha256, etc.)
    pub password: String,
    pub is_admin: bool,
    pub is_disabled: bool,
    pub last_login: DateTime<Utc>,
    pub token_valid_after: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Insertable new user
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    /// raw password (not encrypted)
    pub password: String,
}

/// Parse user from request header
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
        let user = option_to_outcome! { user => (Unauthorized, AccountError::UserIDNotFound) };
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

impl User {
    /// Create a new user and insert into database
    pub fn new(username: String, password: String, conn: &PgConn) -> Result<Self> {
        use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError};
        // hash
        let password = hash::hash(&password);
        let new_user = NewUser {
            username: username.clone(),
            password,
        };

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&**conn)
            .map_err(|e| match e {
                DatabaseError(UniqueViolation, _) => {
                    log::info!("username occupied");
                    Error::Authorization(AccountError::UsernameOccupied(username))
                }
                _ => {
                    log::warn!("Create user failed: {}", e);
                    Error::Database(e)
                }
            })?;

        Ok(user)
    }

    /// Retrieve a valid user from database with given username and password if matched.
    pub fn from(username: &str, password: &str, conn: &PgConn) -> Result<Self> {
        let user = users::table
            .filter(users::username.eq(username))
            .limit(1)
            .get_result::<User>(&**conn)
            .optional()?;
        let user = match user {
            Some(user) => user,
            None => return Err(AccountError::UsernameNotFoundOrPasswordNotMatched.into()),
        };

        if !hash::verify(&password, &user.password) {
            return Err(AccountError::UsernameNotFoundOrPasswordNotMatched.into());
        }

        if user.is_disabled {
            return Err(AccountError::Banned.into());
        }

        Ok(user)
    }

    /// generate new JWT token
    pub fn generate_token(&self, duration: i64, secret: &str) -> String {
        JWT::new(self.id, duration).to_token(secret).unwrap()
    }
}

#[allow(unused)]
mod hash {
    use super::*;
    use base64;

    fn salt() -> Vec<u8> {
        let mut salt = vec![0; 16];
        thread_rng().fill(salt.as_mut_slice());
        salt
    }

    fn hash_with_round_salt(rounds: NonZeroU32, password: &str, salt: &[u8]) -> String {
        let mut checksum = [0u8; PSWD_LENGTH];
        pbkdf2::derive(*PSWD_ALG, rounds, &salt, password.as_bytes(), &mut checksum);
        let base64_salt = base64::encode(salt);
        let base64_checksum = base64::encode(checksum);
        format!(
            "$pbkdf2-sha256${}${}${}",
            rounds, base64_salt, base64_checksum
        )
    }

    /// generate a hex hashed password that is printable and for stored in DB.
    pub fn hash(password: &str) -> String {
        hash_with_round_salt(*PSWD_ROUNDS, password, &salt())
    }

    fn parse_hash(hash: &str) -> Option<(NonZeroU32, Vec<u8>, Vec<u8>)> {
        let split: Vec<&str> = hash.split("$").collect();
        if split.len() != 5 {
            return None;
        }
        // 0:_, 1:pbkdf2-sha256, 2:round, 3:salt, 4:hash
        let round: NonZeroU32 = split[2].parse().expect("Round cannot be parsed");
        // workaround: passlib uses ./ instead of standard +/, so replace all '.' with '+' to make it standard
        let salt = split[3].replace('.', "+");
        let salt = base64::decode(salt).expect("Salt is not base64 encoded");
        let checksum = split[4].replace('.', "+");
        let checksum = base64::decode(checksum).expect("Hash is not base64 encoded");

        Some((round, salt, checksum))
    }

    /// Verify secret against previously generated hash
    pub fn verify(secret: &str, hash: &str) -> bool {
        parse_hash(hash)
            .and_then(|(rounds, salt, previously_derived)| {
                let secret = secret.as_bytes();
                // rounds does not have to be PSWD_ROUNDS
                let iterations = rounds;
                pbkdf2::verify(*PSWD_ALG, iterations, &salt, secret, &previously_derived).ok()
            })
            .is_some()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_hash() {
            let rounds = NonZeroU32::new(1_000).unwrap();
            let secret = "This is a secret".to_string();
            let salt = vec![1, 2, 3, 4];

            let digest = hash_with_round_salt(rounds, &secret, &salt);

            // now verify
            assert!(verify(&secret, &digest));
            // should fail
            assert!(!verify("Wrong password", &digest));
        }

        #[test]
        fn test_passlib_compatible() {
            // should be compatible with passlib generated hash
            assert!(verify("secret", "$pbkdf2-sha256$29000$V4rx/v.fs9a6d.69t3au1Q$7q9uCasYASAT5yOzV7H8Mm0fmJm.0T.lahIejwdNtDY"));
            assert!(verify("secret2", "$pbkdf2-sha256$29000$ACAk5JxzrjVGCKHU.t/7nw$gsRAsU/7UpxKMRCuGuawiqRtDTXrjHKQaIvq0cekQt8"));
            assert!(verify("", "$pbkdf2-sha256$29000$6P2fk9Ja6x2jNEaIcQ7h/A$gSSQF/tSV9gwt53sXe/sOZQaoihnSmPQtTOAXSCu1lE"));
        }
    }
}
