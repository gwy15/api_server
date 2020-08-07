use diesel::prelude::*;

use crate::{PgConn, Result, User};

#[get("/world")]
pub fn index(conn: PgConn) -> &'static str {
    use crate::schema::users::dsl::*;
    use crate::User;

    let results = users
        .filter(is_admin.eq(true))
        .limit(1)
        .load::<User>(&*conn)
        .expect("error loading");
    log::info!("users: {:?}", results);

    // test
    use crate::account::NewUser;
    let _u = NewUser {
        username: "name".into(),
        password: "password".into(),
    };
    // u.insert(&conn);

    "Hello, world!"
}

#[get("/user")]
pub fn user(user: Result<User>) -> Result<String> {
    match user {
        Err(e) => {
            eprintln!("error: {:?}", e);
            format!("failed: {:?}", e);
            Err(e)
        }
        Ok(user) => Ok(format!("Hi! {}", user.username)),
    }
}
