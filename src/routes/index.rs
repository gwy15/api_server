use diesel::pg::PgConnection;
use diesel::prelude::*;

#[get("/world")]
pub fn index() -> &'static str {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let con = PgConnection::establish(&database_url).expect("connection error");

    use crate::models::User;
    use crate::schema::users::dsl::*;

    let results = users
        .filter(is_admin.eq(true))
        .limit(1)
        .load::<User>(&con)
        .expect("error loading");
    println!("users: {:?}", results);
    
    "Hello, world!"
}
