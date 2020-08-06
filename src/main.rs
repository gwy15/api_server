#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;

// make diesel migration module
#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let connection = PgConnection::establish(&database_url).expect("connection error");

    // embedded_migrations::run(&connection);
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("migration failed.");

    rocket::ignite()
        .mount("/hello", routes![gwy15::routes::index::index])
        .register(catchers![gwy15::routes::default::not_found])
        .launch();
}
