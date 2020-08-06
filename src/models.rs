use chrono::prelude::*;

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
