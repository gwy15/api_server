#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[get("/world")]
fn index() -> &'static str {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let con = PgConnection::establish(&database_url).expect("connection error");

    use gwy15::models::User;
    use gwy15::schema::users::dsl::*;

    let results = users
        .filter(is_admin.eq(true))
        .limit(1)
        .load::<User>(&con)
        .expect("error loading");
    println!("users: {:?}", results);
    
    "Hello, world!"
}

#[macro_use] extern crate diesel_migrations;
// make module
embed_migrations!();

fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let connection = PgConnection::establish(&database_url).expect("connection error");
    
    // embedded_migrations::run(&connection);
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).expect("migration failed.");

    rocket::ignite().mount("/hello", routes![index]).launch();
}
