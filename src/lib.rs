#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel_migrations;

// make diesel migration module
embed_migrations!();

pub mod schema;

pub mod errors;
pub use errors::{Error, Result};

mod config;
pub use crate::config::Config;

#[macro_use]
pub mod utils;

pub mod test;

pub mod routes;

pub mod account;
pub use account::User;

// type aliases
mod types {
    use rocket::request::{Form as RForm, FormError};
    pub type Form<'r, T> = std::result::Result<RForm<T>, FormError<'r>>;
}
pub use types::Form;

#[rocket_contrib::database("pg_db")]
pub struct PgConn(diesel::PgConnection);

/// initialize logging
pub fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::info!("logging initialized");
}

/// Create a rocket instance that represents the application
pub fn new_rocket() -> Result<rocket::Rocket> {
    dotenv::dotenv().ok();

    let config = Config::new()?;

    // init_logger();

    let routes = routes![account::routes::login, account::routes::register,];
    let catchers = catchers![routes::catchers::not_found, routes::catchers::unauthorized,];

    // build rocket
    let rocket = rocket::ignite()
        .manage(config)
        .attach(PgConn::fairing())
        .mount("/", routes)
        .register(catchers);

    Ok(rocket)
}

pub fn run_migration(db_con: &diesel::PgConnection) {
    // embedded_migrations::run(&connection);
    embedded_migrations::run_with_output(db_con, &mut std::io::stdout())
        .expect("migration failed.");
    log::info!("Database migration finished.");
}
