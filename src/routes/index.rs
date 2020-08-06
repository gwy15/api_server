use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::PgConn;

#[get("/world")]
pub fn index(conn: PgConn) -> &'static str {
    use crate::models::User;
    use crate::schema::users::dsl::*;

    let results = users
        .filter(is_admin.eq(true))
        .limit(1)
        .load::<User>(&*conn)
        .expect("error loading");
    log::info!("users: {:?}", results);

    "Hello, world!"
}
